{
    description = "CookBook Dev Shell";

    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    };

    outputs = { self, nixpkgs }:
    let
        system = "x86_64-linux";
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {

        devShells.${system}.default = pkgs.mkShell {
            buildInputs = with pkgs; [
                go
                templ
                nodejs_22
                gnumake
                lazygit
                zsh
            ];
            shellHook = ''
                echo "Dev Shell"
                exec zsh
            '';
        };

    };
}
