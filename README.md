# Rust-Webview-demo

0. Set env variable for pkg-config and cross compilation:
```
$ export PKG_CONFIG_x86_64_pc_windows_gnu=/usr/bin/x86_64-linux-gnu-pkg-config
```
1. Build for Windows:
```
$ cargo build --target x86_64-pc-windows-gnu
```
