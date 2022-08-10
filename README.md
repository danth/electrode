# Electrode

There are loads of customisable status bars out there. Some are suffering from
[feature creep](https://en.wikipedia.org/wiki/Feature_creep). Others have
almost nothing built in, and inefficiently call out to scripts written by the
user. Of course, there are good ones too, but they are often closely tied to a
particular Wayland compositor or X window manager.

Electrode is a standalone and efficient status bar for Wayland compositors.
Its only configuration is a single command-line flag.

## Demo

You can try it out using the [Nix package manager](https://nixos.org/) with
Flakes enabled:

```sh
nix run github:danth/electrode
```

Building this repository directly with Cargo works too, provided you have GTK3
development packages available.

## Options

Running Electrode with no arguments will display only the most necessary
information: the clock, volume and battery level.

You can add `--extended` to enable extra statistics such as CPU and memory
usage.

## Installation

This repository provides a [NixOS](https://nixos.org/) module which can be
imported via Flakes. See `nixos.nix` for the available options.

For other distributions, you can create the service by hand at
`/etc/systemd/user/electrode.service`:

```systemd
[Unit]
Description=Electrode status bar

After=graphical-session-pre.target
Before=graphical-session.target
PartOf=graphical-session.target
WantedBy=graphical-session.target

[Service]
ExecStart=/path/to/electrode/bin/electrode
```

Change `ExecStart` to the location of your compiled Electrode binary.
