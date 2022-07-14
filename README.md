# Mouse tracker
## still WIP


### a simple app inspired by [Spud tablet](https://sadwhale-studios.itch.io/spud-tablet)

it animates a hand to show on stream (on [OBS](https://obsproject.com/)) for Vtubers or anyone while they're making art!

# Plans
plans are gonna be in the [TODO issue](https://github.com/BKSalman/mouse_tracker/issues/1)

# What did you use?
I wrote it in Rust
and used:

- bevy for creating the window, and getting window info (like position, width, height), event loop, and handling image rendering

- enigo to get the mouse location ( you could just get it directly from the windows api if you wish )

- bevy_inspector_egui for debugging
