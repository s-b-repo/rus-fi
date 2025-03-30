# RusFi – LoFi Music Player (Audio Only) via yt-dlp

![Description](https://raw.githubusercontent.com/s-b-repo/main/Screenshot_20250330_183350.png)


**RusFi** is a minimal Rust + GTK + GStreamer application that plays the audio-only live LoFi Girl YouTube stream. It calls out to [`yt-dlp`](https://github.com/yt-dlp/yt-dlp) to retrieve the direct audio URL and then streams it via GStreamer. The GUI includes:

- **control buttons**: Play, Pause, Resume, Rewind, and Stop  
- A **Volume Slider**  
- A whimsical **equalizer** visualization that displays random bars

## Features

1. **Audio-only playback** of the live YouTube LoFi Girl stream  
2. **GUI** controls built with GTK  
3. **Random equalizer** pattern to simulate activity has multi-colors and patterns
4. **Volume control** (0.0 = mute, 1.0 = max volume)  
5. **Rewind** attempts -10 seconds


---

## Requirements

1. **Rust** and **Cargo** (v1.60 or later recommended)  
2. **yt-dlp** installed and available in `PATH`  
3. **GStreamer** (1.x) along with relevant plugins (e.g., `gstreamer1.0-plugins-good`, `gstreamer1.0-plugins-bad`, `gstreamer1.0-plugins-ugly`, `gstreamer1.0-libav` – the exact package names vary by distribution)  
4. **GTK** development libraries (e.g., `libgtk-3-dev` or similar, depending on your distro)  

---

Below is an **installation guide** that covers how to install `yt-dlp`, the necessary GStreamer plugins, GTK libraries, Rust, and Cargo on most common Linux distributions. The exact package names may vary slightly depending on your distro, but these instructions will give you a strong starting point.

---

## 1. Install Rust & Cargo

**Rust** (and the associated `cargo` build tool) can be installed a few ways. The official and easiest method is via [rustup](https://rustup.rs):

```
# Download and run the rustup script:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the onscreen instructions and log out / log in if needed.
# Ensure your ~/.cargo/bin is in your PATH:
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.rc
source ~/.rc
```

You can verify Rust is installed correctly with:
```
rustc --version
cargo --version
```

---

## 2. Install `yt-dlp`

### Option A: Using your distribution’s package manager

- **Ubuntu/Debian**:  
  ```
  sudo apt update
  sudo apt install yt-dlp
  ```

- **Fedora**:  
  ```
  sudo dnf install yt-dlp
  ```

- **Arch Linux**:  
  ```
  sudo pacman -S yt-dlp
  ```

### Option B: Install via Python / Pip

If your distribution’s packages are outdated or unavailable, you can install the latest `yt-dlp` with `pip`:

```
python3 -m pip install --upgrade yt-dlp
```

Make sure that the scripts installed by `pip` (e.g. `~/.local/bin`) are in your PATH:

```
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.rc
source ~/.rc
```

You can check the version with:
```
yt-dlp --version
```

---

## 3. Install GStreamer & Plugins

The LoFi YouTube stream often comes as an M3U8, MP4, or WebM container with AAC/Opus audio, so you may need **all** relevant GStreamer plugins (good, bad, ugly, libav). The names differ by distro:

- **Ubuntu/Debian**:
  ```
  sudo apt-get install gstreamer1.0-tools \
                       gstreamer1.0-plugins-base \
                       gstreamer1.0-plugins-good \
                       gstreamer1.0-plugins-bad \
                       gstreamer1.0-plugins-ugly \
                       gstreamer1.0-libav
  ```

- **Fedora**:
  ```
  sudo dnf install gstreamer1-plugins-base \
                   gstreamer1-plugins-good \
                   gstreamer1-plugins-bad-free \
                   gstreamer1-plugins-ugly-free \
                   gstreamer1-libav
  ```

- **Arch Linux**:
  ```
  sudo pacman -S gstreamer gst-plugins-base gst-plugins-good gst-plugins-bad gst-plugins-ugly gst-libav
  ```

> **Tip**: To confirm GStreamer is installed, try:
> ```
> gst-launch-1.0 --version
> ```

---

## 4. Install GTK Development Libraries

This project uses **GTK 3**. You need the GTK **development** headers for building Rust GTK applications.

- **Ubuntu/Debian**:
  ```
  sudo apt-get install libgtk-3-dev
  ```

- **Fedora**:
  ```
  sudo dnf install gtk3-devel
  ```

- **Arch Linux**:
  ```
  sudo pacman -S gtk3
  ```
  (Arch typically includes headers in the same package.)

Check that the library is installed by seeing if `pkg-config` can locate GTK 3:
```
pkg-config --cflags gtk+-3.0
```

---

## 5. Build & Install RusFi

After you have:

1. **Rust** (and `cargo`)
2. **yt-dlp**
3. **GStreamer** and necessary plugins
4. **GTK** dev libraries

You can now build **RusFi** from source. If you cloned the repository:

```
cd rusfi  # wherever you cloned this repository
cargo install --path .
```

The above command will:

- Build RusFi in release mode
- Install the resulting binary (`rusfi`) into your `~/.cargo/bin` directory by default

Ensure `~/.cargo/bin` is in your `PATH`. If not, add it:

```
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.rc
source ~/.rc
```

---

## 6. Run the Player

Once installed, you can run from any terminal by typing:

```
rusfi
```

A window should appear with:

1. **7 buttons**: **Play**, **Pause**, **Resume**, **Rewind**, **FastFwd**, **Skip Live**, **Stop**  
2. **Volume slider** below the buttons  
3. A random-bar “equalizer”  

Click **Play** to start the **LoFi Girl** stream audio.

- **7 buttons**:  
  - **Play**: Starts the audio  
  - **Pause**: Pauses playback  
  - **Resume**: Resumes from pause  
  - **Rewind**: Tries seeking back 10 seconds (if DVR is supported)  
  - **FastFwd**: Seeks forward 10 seconds (if DVR is supported)  
  - **Skip Live**: Attempts to jump to the live edge  
  - **Stop**: Sets the pipeline to Null (completely stops playback)  

- **Volume slider** (0.0 → 1.0)  
- **Animated equalizer**: A purely cosmetic bar animation  

---

## Code Overview

1. **Rust + Cargo**: The code is in `src/main.rs`.  
2. **GStreamer**: We use the GStreamer crate to create a `playbin` pipeline and manage playback states.  
3. **yt-dlp**: We shell out to `yt-dlp` using a `std::process::Command` call to fetch the direct audio URL for the Lofi Girl live stream.  
4. **GTK**: We use the GTK 3.x bindings in Rust to build the GUI:  
   - **Buttons** for each player action.  
   - A **Scale** widget for volume control.  
   - A **DrawingArea** to animate random bar heights (a simple “equalizer”).  

---

## Troubleshooting

1. **yt-dlp Not Found**: Make sure `yt-dlp` is installed and in your `PATH`. Test with `yt-dlp --version`.  
2. **GStreamer Plugin Errors**: If GStreamer can’t play the extracted audio URL, ensure you have `gst-libav` and the “bad/ugly” plugin sets installed.  
3. **Live Stream Doesn’t Seek**: Some streams do not allow rewinding or skipping forward. The LoFi Girl stream usually has a small DVR window, but the behavior can vary.  
4. **No Audio**: Check your system volume, or see if the volume slider is at 0.0 in the application.  
5. **GUI Crashes**: Make sure you have the correct GTK dev libraries installed and the environment variables set up for X11/Wayland (depending on your system).  

---

## License

This project is available under the terms of the **MIT License** (or your preferred open-source license).  

Feel free to modify, share, and enjoy some LoFi tunes!

---

**Thanks for using RusFi!** If you find this helpful, please consider giving the repository a ⭐ on GitHub. Enjoy your relaxing music!
