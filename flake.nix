{
  description = "Development tools for btrfs-diskformat";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
    in
    {
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt);

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages =
              with pkgs;
              [
                bash
                coreutils
                cargo
                rustc
                rustfmt
                clippy
                rust-analyzer
                just
                python3
                shellcheck
                shfmt
                nixfmt
              ]
              ++ lib.optionals stdenv.hostPlatform.isLinux [
                btrfs-progs
                util-linux
              ];
          };
        }
      );
    };
}
