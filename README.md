# XROAGWEM
Window manager for X11 written in Rust. Pretty usable (for a single display as of now), still many bugs.

The goal is to support all of the features from [awesomewm](https://github.com/awesomeWM/awesome) I use on daily basis (my config [here](https://github.com/laudominik/awesome-config)) and switch to it:
* auto cascade tiling ✔
* fullscreen window ✔
* floating window ✔
* wallpaper ✔
* spawning some scripts with shell on startup ✔
* top bar ✔
* mouse motions ✔
* multiple workspaces ✔
* keybindings ✔
* resetting the wm
* two screens support

as you can probably tell, there's not too much functionality I care about.

## Installation
Clone, then
```
cargo build --release
```
Add the `exec <path to xroagwem binary>` to your xinitrc (it's in `target/release/xroagwem`).

## Configuration
See [config.rs](src/config.rs) for the wm configuration, it requires a rebuild on each config change.

## References
Heavily inspired by [dwm](https://dwm.suckless.org/).
