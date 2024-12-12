with (import <nixpkgs> {});
pkgs.mkShell {
  buildINputs = [ pkgs.rustup ];
}
