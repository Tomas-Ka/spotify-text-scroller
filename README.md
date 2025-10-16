# spotify-text-scroller
This project was made with waybar in mind, here's my extremely simple module:
```json
"custom/spotify": {
  "exec": "scr0ll"
},
```

### Usage
```
Usage: scr0ll [OPTIONS]

Options:
  -l, --length <LENGTH>            Length of the outputed value [default: 20]
  -d, --delay <DELAY>              ms of time to wait between outputs [default: 300]
  -w, --wait-time <WAIT_TIME>      ms of time to wait before scrolling [default: 300]
  -c, --custom-icon <CUSTOM_ICON>  custom icon [default: "\u{f1bc} "]
  -h, --help                       Print help
  -V, --version                    Print version
```

### Installing
Make sure to install [playerctl](https://github.com/altdesktop/playerctl) for your distro, or else
it won't work. Blame [this crate](https://crates.io/crates/playerctl).

```sh
git clone https://github.com/S0raWasTaken/spotify-text-scroller
cargo install --path spotify-text-scroller
```

### Preview
![](resources/scr0ll.gif)