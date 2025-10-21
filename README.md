# spotify-text-scroller

This project was made with waybar in mind, here's my extremely simple module:

```json
"custom/spotify": {
  "exec": "scr0ll"
},
```

## Usage

```help
Usage: spotify-scroller [OPTIONS]

Options:
  -l, --length <LENGTH>        Length of the output value [default: 20]
  -d, --delay <DELAY>          Time to wait between outputs in ms [default: 300]
  -w, --wait-time <WAIT_TIME>  Time to wait before scrolling in ms [default: 300]
  -i, --icon <ICON>            Icon [default: ]
      --disable-icon           Disable icon
      --disable-author         Disable author
  -h, --help                   Print help
  -V, --version                Print version
```

## Installing

Make sure to install [playerctl](https://github.com/altdesktop/playerctl) for
your distro, or else it won't work. Blame [this crate](https://crates.io/crates/playerctl).

```sh
git clone https://github.com/Tomas-Ka/spotify-text-scroller
cargo install --path spotify-text-scroller
```

## Preview

![Gif of a track name and artist scrolling in a taskbar](resources/scr0ll.gif)
