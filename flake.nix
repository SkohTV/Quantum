{
inputs.poetry2nix.url = "github:nix-community/poetry2nix";


outputs = { self, nixpkgs, poetry2nix }:

let
  system = "x86_64-linux";
  pkgs = nixpkgs.legacyPackages.${system};

  inherit (poetry2nix.lib.mkPoetry2Nix { inherit pkgs; }) mkPoetryApplication mkPoetryEnv;
  pythonApp = mkPoetryApplication { };
  pythonEnv = mkPoetryEnv { };
  # pythonApp = mkPoetryApplication { projectDir = ./.; };
  # pythonEnv = mkPoetryEnv { projectDir = ./.; };

in {

  devShells.${system}.default = pkgs.mkShell {
    packages = [ pkgs.python312Full pythonEnv ];
  };

  apps.${system}.default = {
    type = "app";
    program = "${pythonApp}/bin/<script>";
  };

};
}
