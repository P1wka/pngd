# pngd
- ** SDL2 + Rust - Random Pixel Drawer for PNG files **
- This project reads a PNG file and draws randomly selected pixels in an SDL2 window using Rust.

## Requirements

- Rust
- SDL2 library and `SDL2.dll` (for Windows)
- PNG file (`file.png`)


## Installation

1. Clone the repository:

```bash
git clone https://github.com/P1wka/pngd.git
cd pngd
```

2. Copy the SDL2 DLL file to the project directory (Windows):

```bash
SDL2.dll
```

3. Add your PNG file to assets/png folder.

4. Build the project and fetch dependencies:

```bash
cargo build --release, or
cargo build & cargo run
```

## Usage

- When the program runs, it will ask you for the number of pixels to draw.
- An SDL2 window will open and display randomly selected pixels.

## Notes
- The ```target/``` folder is not included in Git.
- Drawing too many pixels for large PNG files may affect performance.
- Make sure the SDL2 DLL file is in the same directory as the executable.
- Advice: Attach .png files with a maximum size of 800x600.

Thanks for using **pngd** =)
