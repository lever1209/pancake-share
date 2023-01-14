# pancake share

a simple p2p file/media sharing program

---

### in future:

there will be a way to send messages to other clients even if they are offline/have no way to receive the message

optional encryption of some kind, it will start off very basic, but as i learn more about encryption it will get better and cover more ground

# Build instructions:

linux:
  execute the [build script](build) like so `./build windows` or `./build linux`

windows:
  the command listed below may not be as up to date as the one in the build script, if you have issues please refer to the build script
  `cargo build --features=windows-native --release --target=x86_64-pc-windows-gnu`
