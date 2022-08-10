# Electrode

There are loads of customisable status bars out there. Some are suffering from
[feature creep](https://en.wikipedia.org/wiki/Feature_creep). Others have
almost nothing built in, and inefficiently call out to scripts written by the
user. Of course, there are good ones too, but they are often closely tied to a
particular Wayland compositor or X window manager.

Electrode is a standalone and efficient status bar for Wayland compositors.
Its only configuration is a single command-line flag.

## Installation

You can try it out using the [Nix package manager](https://nixos.org/) with
Flakes enabled:

```sh
nix run github:danth/electrode
```

Building this repository directly with Cargo works too, provided you have GTK3
development packages available.

## Configuration

Running Electrode with no arguments will display only the most necessary
information: the clock, volume and battery level.

You can add `--extended` to enable extra statistics such as CPU and memory
usage.

## Why the name?

It looks a bit like the end of this phone battery.

![A Nokia battery](https://i.stack.imgur.com/upgXL.jpg)
