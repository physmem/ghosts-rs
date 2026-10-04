# ghosts-rs
A simple internal dlc unlocker & wallhack for Call of Duty: Ghosts

This unlocks EVERYTHING in the game as it hooks `LiveStorage_IsItemUnlockedFromTable_LocalClient` and just returns true. Do note that this is only a temporary unlock, uninjecting/restarting your game will restore your original unlocks
# Building
Simply run `cargo build` and find the output binary in `target/debug/ghosts_rs.dll`

# Credits
[a2x](https://github.com/a2x/) - everything in /utils pretty much
