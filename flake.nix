{ outputs = { self, nixpkgs }:
{

devShells.default =
  nixpkgs.mkShell {
    packages = [
      (nixpkgs.python312.withPackages (ps: with ps; [
        virtualenv
        pip

        discordpy
      ]))
    ];
  };

};
}
