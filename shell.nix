{ pkgs ? import <nixpkgs> {}, ... }: pkgs.mkShell {
  name = "dawless";
  buildInputs = with pkgs; [ ncurses ];
}
