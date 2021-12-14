with import <nixpkgs> { };
mkShell {
  shellHook = ''
    set -a
    source ${toString ./.env}
    set +a
  '';
}
