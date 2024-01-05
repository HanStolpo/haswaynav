# Summary

Small command line utility for customized navigation in sway.

The following changes the focused window to the one on the left skipping over
any sibbling windows in a tabbed or stacked container.

```bash
haswaynav focus left
```

Where as the default below would select the next tab if there was a tabbed
sibling to the left of the focused window.

```bash
swaysm 'focus left'
```

# Installation

## NixOS

If you are using NixOS with flakes then you can use the overlay supplied by
the `flake.nix` to add `haswaynav` to the list of available packages.

e.g.

```nix
{
  description = "example";

  inputs = {
    nixpkgs.url = github:NixOS/nixpkgs/nixos-23.11;
    haswaynav.url = path:/home/handre/dev/haswaynav;
  };

  outputs = { self, nixpkgs, haswaynav }: {
    nixosConfigurations.example = nixpkgs.lib.nixosSystem {
      system = "x86_64-linux";
      modules = [
        ({ pkgs, ... }: {
          nixpkgs = {
            overlays = [ haswaynav.overlays.default ];
          };
        })

        ./configuration.nix
      ];
    };
  };
}
```

## Build from source

If you have a Rust tool chain installed then you can build the utility from source.

e.g.

```bash
git clone https://github.com/hanstolpo/haswaynav
cd haswaynav
cargo install --path .
```
