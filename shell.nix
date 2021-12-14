with import <nixpkgs> { };
mkShell {
  shellHook = ''
    source ${toString ./.env}
  '';
}
