# dinf

🛠️ A powerful command-line tool that scans a specified folder path and provides a detailed summary, including the total number of folders, files, and their combined size.

## 🚀 Features

- **Folder and File Count**: Quickly calculates the total number of folders and files in the specified path.
- **Size Calculation**: Displays the total size in bytes, KB, MB, or GB, depending on the size.
- **Cross-Platform**: Works seamlessly on Windows, macOS, and Linux.

## 📦 Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed. If not, install it via [rustup](https://rustup.rs/).

1. Clone the repository:

   ```bash
   $ git clone https://github.com/vilsonfcastilho/dinf.git
   $ cd dinf
   ```

2. Build the project:

   ```bash
   $ cargo build --release
   ```

3. (Optional) Add the binary to your system's PATH:

   For Linux/macOS:

   ```bash
   $ cp target/release/dinf /usr/local/bin/dinf
   ```

   For Windows

   ```bash
   $ setx PATH "%PATH%;C:\path\to\your\project\target\release"
   ```

## 🛠️ Usage

Run the tool with a folder path as an argument:

    ```bash
      $ cargo run {{path}}
    ```

Example:

    ```bash
      $ cargo run /path/to/folder
    ```

Output:

    ```bash
      Status ✨
      folders: 5
      files: 10
      size: 1.45 MB
    ```

## 🧑‍💻 How It Works

The tool:

1. Scans the folder path provided as an argument.
2. Recursively counts all subfolders and files.
3. Calculates the total size of files in the folder.

---

Made with ♥ by Vilson Castilho
