{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }@inputs: {
    defaultPackage.x86_64-linux =
      # Notice the reference to nixpkgs here.
      with import nixpkgs { system = "x86_64-linux"; };
      rustPlatform.buildRustPackage rec {
        name = "projektwahl-wasm";
        src = "${self}/projektwahl-wasm";
        target = "wasm32-unknown-unknown";
        cargoHash = "sha256-2+DQqydNFFbF6Nc/x7ghjkg6qV/6w+cxyZLLluOML4o=";
      };
  };
}