# Erae
A lightweight Emacs-like editor

## Overview
Erae is a lightweight and simple editor that follows Emacs' operability while optimizing some functions for efficiency. It retains major Emacs keybindings but introduces some unique optimizations.

## Features
- Emacs-like keybindings
- Lightweight and fast
- Provides essential file editing functions
- Intuitive operation

## Differences in Keybindings between Emacs and Erae

| Action | Emacs | Erae |
|:-----------|------------:|:------------:|
|   Save File    |    C-x C-s     |     C-s     |
| Exit     |      C-x C-c |    C-q    |

Most basic operations remain the same as in Emacs.

(C-o, C-f, C-b, C-d, C-n, C-p all work the same as in Emacs.)

## Installation
```sh
# Clone from GitHub and build
git clone https://github.com/kaedehito/Erae.git
cd Erae
cargo install --path .
```

## Usage
```sh
Erae <filename>
```

## License
This software is provided under the MIT License.

