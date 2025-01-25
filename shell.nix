{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  name = "Rust";

  # Specify dependencies here
  buildInputs = with pkgs; [
    #unstable.rustup
    rustup
    file
    elfutils
    elf2uf2-rs
    #unstable.wasm-bindgen-cli

  ];

  # Optional: Environment variables for Rust or Dioxus
#  RUSTFLAGS = "-C target-feature=+crt-static"; # Example: static linking, can be customized
#  CARGO_HOME = ".cargo";                       # Custom Cargo home directory
#  CARGO_INCREMENTAL = "1";                     # Enable incremental compilation for faster builds

  # Optional: Add any specific instructions or shell hooks here
  shellHook = ''
    echo "Welcome to the $name development shell!"
    echo "All necessary libraries and tools are installed."
  '';
}
