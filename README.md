<img width="100" height="100" alt="logo" src="https://github.com/user-attachments/assets/7a99012c-2f55-40b3-9019-a53204ba594a"/> 

A music player in the terminal.

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

This repository is written by humans and please do not make a pull request containing direct output of generative tools. As those tools might be trained on projects with questionable licenses. However, it is troublesome for us to tell the difference between generated and written code thus please use your own discretion. Let's use those tools as tools, not as a replacement of you.

We are not claiming this project will be better off without those tools. Probably, opposite. This small project can be developed much faster and more efficiently but can we claim it as our own? More importantly, what's the joy in it? 
