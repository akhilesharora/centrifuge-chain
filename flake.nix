{
  description = "Nix package for centrifuge-chain";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-20.09;
  };

  outputs = inputs:
    let
      name = "centrifuge-chain";
      major = "2.0.0";
      version = "${major}-${commit-substr}";
      system = "x86_64-linux";

      pkgs = inputs.nixpkgs.legacyPackages.${system};

      # This evaluates to the first 6 digits of the git hash of this repo's HEAD
      # commit, or to "dirty" if there are uncommitted changes.
      commit-substr = builtins.substring 0 6 (inputs.self.rev or "dirty");

      # This is a mock git program, which just returns the commit-substr value.
      # It is called when the build process calls git. Instead of the real git,
      # it will find this one.
      git-mock = pkgs.writeShellScriptBin "git" ''
        echo ${commit-substr}
      '';

      # srcFilter is used to keep out of the build non-source files,
      # so that we only trigger a rebuild when necessary.
      srcFilter = path: type:
        let
          p = baseNameOf path;
        in
          !(
            # ignore CI directories
            (type == "directory" && (p == ".github" || p == "ci")) ||
            # ignore cargo files
            (type == "directory" && (p == "target")) || p == ".cargo" ||
            # ignore CI files
            p == ".travis.yml" || p == "cloudbuild.yaml" ||
            # ignore flake.(nix|lock)
            p == "flake.nix" || p == "flake.lock" ||
            # ignore docker files
            p == ".dockerignore" || p == "docker-compose.yml" ||
            # ignore misc
            p == "rustfmt.toml" || p == ".idea" || p == ".vscode"
          );

    in
    {
      packages.${system} = {
        # This is the native package.
        centrifuge-chain = pkgs.rustPlatform.buildRustPackage {
          pname = name;
          inherit version;

          # This applies the srcFilter function to the current directory, so
          # we don't include unnecessary files in the package.
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = srcFilter;
            name = "${name}-source";
          };
          # This is a hash of all the Cargo dependencies.
          cargoSha256 = "sha256-ulzzofKBqw4RUwwBmFKvgfCZ1ZeuULvCHLEQVzZrKBk=";

          nativeBuildInputs = with pkgs; [ clang git-mock pkg-config ];
          buildInputs = [ pkgs.openssl ];

          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          BUILD_DUMMY_WASM_BINARY = 1;

          doCheck = false;
        };

        # This is the Docker image.
        dockerImage = pkgs.dockerTools.buildLayeredImage {
          name = "centrifugeio/${name}";
          tag = version;

          contents = inputs.self.defaultPackage.${system};

          config = {
            ExposedPorts = {
              "30333/tcp" = { };
              "9933/tcp" = { };
              "9944/tcp" = { };
            };
            Volumes = {
              "/data" = { };
            };
            Entrypoint = [ "centrifuge-chain" ];
          };
        };
      };

      defaultPackage.${system} = inputs.self.packages.${system}.centrifuge-chain;
    };
}