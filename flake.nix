{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }@inputs: {
    defaultPackage.x86_64-linux =
      # Notice the reference to nixpkgs here.
      with import nixpkgs { system = "x86_64-linux"; };
      stdenv.mkDerivation rec {
        name = "projektwahl-wasm";
        src = "${self}/projektwahl-wasm";

        nativeBuildInputs = [ rustup pkg-config ];
        buildInputs = [ openssl ];
      };
  };
}