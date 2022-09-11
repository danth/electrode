# Electrode

There are loads of customisable status bars out there. Some are suffering from
[feature creep](https://en.wikipedia.org/wiki/Feature_creep). Others pull
information from user supplied scripts, which can become highly inefficient.

I wanted something which would stay out of the way, both visually and in terms
of its power consumption. Electrode has a negligible impact on the system as
all of its features are written in Rust, and compiled into a single binary
of around 1MiB.

![Screenshot of the status bar](./screenshot.webp)

Unlike a traditional status bar which takes up a fixed space on the screen,
Electrode blends in with the background as much as possible. This looks
particularly good in combination with [Wayfire](https://wayfire.org/), but it
works on any Wayland compositor. A careful choice of wallpaper is required as
some images can make the text difficult to read.

## Demo

You can try Electrode using the [Nix package manager](https://nixos.org/) with
Flakes enabled:

```sh
nix run github:danth/electrode
```

Building this repository directly with Cargo works too, provided you have GTK3
development packages available.

## Options

Use the `--color` flag to change the text to suit your wallpaper image.
This accepts any CSS color format.

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
