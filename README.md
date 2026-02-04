**🦀 Rust File Renamer**

A lightweight, high-performance GUI file renamer built with **Rust** and **eframe (egui)**. This tool allows you to batch rename files using dynamic pattern matching and provides a real-time "dry run" preview.

## **🚀 How it Works**

The application uses a **Tag-Based Matching** system. You define how the current filename is structured using {tags}, and the app extracts those values to rebuild the new filename.

### **The Pattern System**

* **Current Pattern:** Tells the app how to "read" your existing files.  
* **New Pattern Template:** Tells the app how to "write" the new names.

#### **Example:**

Imagine you have a folder full of music files named:

Artist - SongTitle (2024).mp3

1. **Current Pattern:** `{artist} - {title} ({year}).{ext}`
2. **New Pattern:** `{year} - {title} [By {artist}].{ext}`

**Result:**

`2024 - SongTitle [By Artist].mp3`

**🛠️ Building from Source**

Ensure you have the Rust toolchain installed. If building for other architectures from your Kubuntu laptop, you will need the specific targets and linkers.

### **1. Linux (Desktop/Laptop)**

For your Victus laptop or any standard 64-bit Linux distro.

Bash

`cargo build --release`

**Output:** `target/release/rust_file_renamer`

### **2. Raspberry Pi 4 (ARM64)**

If you are cross-compiling from your laptop to run on your Ubuntu Server Pi:

1. Add the target:  
   `rustup target add aarch64-unknown-linux-gnu`

2. Build:  
   `cargo build --release --target aarch64-unknown-linux-gnu`

**Output:** target/aarch64-unknown-linux-gnu/release/rust_file_renamer

### **3. Windows (Cross-Compile from Linux)**

To generate a .exe while staying on Kubuntu:

1. Install the Windows target and MinGW toolchain:  
   `rustup target add x86_64-pc-windows-gnu  
   sudo apt install binutils-mingw-w64 g++-mingw-w64`

2. Build:    
   `cargo build --release --target x86_64-pc-windows-gnu`

**Output:** `target/x86_64-pc-windows-gnu/release/rust_file_renamer.exe`

**📦 Dependencies**

If building on a fresh Linux install (like your Pi), you may need the following system packages:

Bash

sudo apt install build-essential libxcb-shape0-dev libxcb-xfixes0-dev libx11-dev libasound2-dev

## **📝 License**

MIT / Apache-2.0
