# Electrode

Electrode is a no-configuration status bar for Wayland compositors.

## Installation

You can try it out using the [Nix package manager](https://nixos.org/) with
Flakes enabled:

```sh
nix run github:danth/electrode
```

Building this repository directly with Cargo should work, too, provided you
have GTK3 development packages installed.

## Usage

Running Electrode with no arguments will display only the most necessary
information: the clock, volume and battery level.

You can add `--extended` to the arguments to enable extra statistics such as
CPU and memory usage.

## Why the name?

It looks a bit like the end of this phone battery.

![A Nokia battery](https://i.stack.imgur.com/upgXL.jpg)
