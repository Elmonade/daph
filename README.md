<img width="100" height="100" alt="logo" src="https://github.com/user-attachments/assets/7a99012c-2f55-40b3-9019-a53204ba594a"/> 

A music player in the terminal.

## Structure
Here, [TEA](https://guide.elm-lang.org/architecture/) is the way. Each user input is mapped to a set of pre-defined commands the player can understand. The command is then sent to the `update` method which modifies the `model`. Finally, the `view` reflects the changes.  

## Controls
The keybindings are inspired by vim. However, certain keys are avoided to accidentally skip or seek a track. As for why we have two play button, the `:` button is for loading the track into the sink so it's mostly used when picking a new song manually. While, `Space` is for controlling whats already in the sink. This might be a bit roundabout way of doing it so any suggestions on regarding control scheme is welcome, addition to normal code-related issues. 

The search and delete functionalities are still under construction.

| Category | Key | Action |
|----------|-----|--------|
| **Navigation** | `j` | Move down in track list |
| | `k` | Move up in track list |
| **Playback** | `:` | Play/pause highlighted track |
| | `Space` | Play/pause the track in the sink |
| | `n` | Next track |
| | `p` | Previous track |
| **Seeking** | `<` | Seek backward 5 seconds |
| | `>` | Seek forward 5 seconds |
| **Volume** | `K` | Volume up |
| | `J` | Volume down |
| **Search** | `/` | Enter search mode |
| | Type | Search by song title or artist |
| | `Enter` | Submit search |
| | `Esc` | Exit search mode |
| **Other** | `D` | Delete selected track from playlist |
| | `Esc` | Quit application |

Supports MP3, FLAC, and WAV audio files.
Control scheme and supported file formats are subject to change. 

Written by a mere mortals.
