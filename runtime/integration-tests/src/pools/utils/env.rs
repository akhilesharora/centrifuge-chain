// Copyright 2021 Centrifuge Foundation (centrifuge.io).
//
// This file is part of the Centrifuge chain project.
// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).
// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use crate::chain::centrifuge::{
	Block as CentrifugeBlock, RuntimeApi as CentrifugeRtApi, PARA_ID, WASM_BINARY as CentrifugeCode,
};
use crate::chain::relay::{Runtime as RelayRt, RuntimeApi as RelayRtApi, WASM_BINARY as RelayCode};
use frame_support::traits::GenesisBuild;
use fudge::digest::FudgeBabeDigest;
use fudge::{
	digest::DigestCreator,
	inherent::{
		CreateInherentDataProviders, FudgeDummyInherentRelayParachain, FudgeInherentParaParachain,
		FudgeInherentTimestamp,
	},
	EnvProvider, ParachainBuilder, RelaychainBuilder,
};
use polkadot_core_primitives::{Block as RelayBlock, Header as RelayHeader};
use polkadot_parachain::primitives::Id as ParaId;
use sc_executor::{WasmExecutionMethod, WasmExecutor};
use sc_service::{SpawnTaskHandle, TaskManager};
use sp_consensus_babe::digests::CompatibleDigestItem;
use sp_core::H256;
use sp_runtime::{generic::BlockId, DigestItem, Storage};
use std::sync::Arc;
use tokio::runtime::Handle;

/// The type that CreatesInherentDataProviders for the relay-chain.
/// As a new-type here as otherwise the TestEnv is badly
/// readable.
#[allow(unused)]
type RelayCidp = Box<
	dyn CreateInherentDataProviders<
		RelayBlock,
		(),
		InherentDataProviders = (
			FudgeInherentTimestamp,
			sp_consensus_babe::inherents::InherentDataProvider,
			sp_authorship::InherentDataProvider<RelayHeader>,
			FudgeDummyInherentRelayParachain<RelayHeader>,
		),
	>,
>;

/// The type that CreatesInherentDataProviders for the para-chain.
/// As a new-type here as otherwise the TestEnv is badly
/// readable.
#[allow(unused)]
type CentrifugeCidp = Box<
	dyn CreateInherentDataProviders<
		CentrifugeBlock,
		(),
		InherentDataProviders = (
			FudgeInherentTimestamp,
			sp_consensus_babe::inherents::InherentDataProvider,
			FudgeInherentParaParachain,
		),
	>,
>;

/// The type creates digests for the chains.
#[allow(unused)]
type Dp = Box<dyn DigestCreator + Send + Sync>;

// TODO: Solve the issue that currently one can only manually add the parachain-ids here
//       We would need the macro to take the constant
#[fudge::companion]
pub struct TestEnv {
	#[fudge::relaychain]
	relay: RelaychainBuilder<RelayBlock, RelayRtApi, RelayRt, RelayCidp, Dp>,
	#[fudge::parachain(2000)]
	centrifuge: ParachainBuilder<CentrifugeBlock, CentrifugeRtApi, CentrifugeCidp, Dp>,
}

#[allow(unused)]
pub fn test_env_default(handle: SpawnTaskHandle) -> TestEnv {
	test_env(handle, None, None)
}

#[allow(unused)]
pub fn test_env_with_relay_storage(handle: SpawnTaskHandle, storage: Storage) -> TestEnv {
	test_env(handle, Some(storage), None)
}

#[allow(unused)]
pub fn test_env_with_centrifuge_storage(handle: SpawnTaskHandle, storage: Storage) -> TestEnv {
	test_env(handle, None, Some(storage))
}

#[allow(unused)]
pub fn test_env_with_both_storage(
	handle: SpawnTaskHandle,
	relay_storage: Storage,
	centrifuge_storage: Storage,
) -> TestEnv {
	test_env(handle, Some(relay_storage), Some(centrifuge_storage))
}

fn test_env(
	handle: SpawnTaskHandle,
	relay_storage: Option<Storage>,
	centrifuge_storage: Option<Storage>,
) -> TestEnv {
	// Build relay-chain builder
	let relay = {
		let mut provider = EnvProvider::<
			RelayBlock,
			RelayRtApi,
			WasmExecutor<sp_io::SubstrateHostFunctions>,
		>::with_code(RelayCode.unwrap());

		// We need to HostConfiguration and use the default here.
		provider.insert_storage(
			polkadot_runtime_parachains::configuration::GenesisConfig::<RelayRt>::default()
				.build_storage()
				.expect("ESSENTIAL: GenesisBuild must not fail at this stage."),
		);

		if let Some(storage) = relay_storage {
			provider.insert_storage(storage);
		}

		let (client, backend) = provider.init_default(
			WasmExecutor::new(WasmExecutionMethod::Interpreted, Some(8), 8, None, 2),
			Box::new(handle.clone()),
		);
		let client = Arc::new(client);
		let clone_client = client.clone();

		let cidp = Box::new(move |parent: H256, ()| {
			let client = clone_client.clone();
			let parent_header = client
				.header(&BlockId::Hash(parent.clone()))
				.unwrap()
				.unwrap();

			async move {
				let uncles =
					sc_consensus_uncles::create_uncles_inherent_data_provider(&*client, parent)?;

				let timestamp =
					FudgeInherentTimestamp::new(0, std::time::Duration::from_secs(6), None);

				let slot =
					sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
						timestamp.current_time(),
						std::time::Duration::from_secs(6),
					);

				let relay_para_inherent = FudgeDummyInherentRelayParachain::new(parent_header);
				Ok((timestamp, slot, uncles, relay_para_inherent))
			}
		});

		let dp = Box::new(move || async move {
			let mut digest = sp_runtime::Digest::default();

			let slot_duration = pallet_babe::Pallet::<RelayRt>::slot_duration();
			digest.push(<DigestItem as CompatibleDigestItem>::babe_pre_digest(
				FudgeBabeDigest::pre_digest(
					FudgeInherentTimestamp::get_instance(0).current_time(),
					std::time::Duration::from_millis(slot_duration),
				),
			));

			Ok(digest)
		});

		RelaychainBuilder::<_, _, RelayRt, RelayCidp, Dp>::new(
			handle.clone(),
			backend,
			client,
			cidp,
			dp,
		)
	};

	// Build parachain-builder
	let centrifuge = {
		let mut provider = EnvProvider::<
			CentrifugeBlock,
			CentrifugeRtApi,
			WasmExecutor<sp_io::SubstrateHostFunctions>,
		>::with_code(CentrifugeCode.unwrap());

		if let Some(storage) = centrifuge_storage {
			provider.insert_storage(storage);
		}

		let (client, backend) = provider.init_default(
			WasmExecutor::new(WasmExecutionMethod::Interpreted, Some(8), 8, None, 2),
			Box::new(handle.clone()),
		);
		let client = Arc::new(client);
		let para_id = ParaId::from(PARA_ID);
		let inherent_builder = relay.inherent_builder(para_id.clone());

		let cidp = Box::new(move |_parent: H256, ()| {
			let inherent_builder_clone = inherent_builder.clone();
			async move {
				let timestamp =
					FudgeInherentTimestamp::new(0, std::time::Duration::from_secs(6), None);

				let slot =
					sp_consensus_babe::inherents::InherentDataProvider::from_timestamp_and_duration(
						timestamp.current_time(),
						std::time::Duration::from_secs(6),
					);
				let inherent = inherent_builder_clone.parachain_inherent().await.unwrap();
				let relay_para_inherent = FudgeInherentParaParachain::new(inherent);
				Ok((timestamp, slot, relay_para_inherent))
			}
		});
		let dp = Box::new(move || async move { Ok(sp_runtime::Digest::default()) });

		ParachainBuilder::<_, _, CentrifugeCidp, Dp>::new(handle.clone(), backend, client, cidp, dp)
	};

	TestEnv::new(relay, centrifuge).unwrap()
}

pub fn task_manager(tokio_handle: Handle) -> TaskManager {
	TaskManager::new(tokio_handle, None).expect("ESSENTIAL: TaskManager must exist for tests.")
}

/// Pass n_blocks on the parachain-side!
pub fn pass_n(n: u64, env: &mut TestEnv) -> Result<(), ()> {
	for _ in 0..n {
		env.evolve()?;
	}

	Ok(())
}
