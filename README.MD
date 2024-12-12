# Service Desk Kiosk

Service Desk Kiosk is a cross-platform desktop application built using the [Tauri Framework](https://tauri.app/). This application is designed to create a kiosk-like interface with specific configurations and features for ease of use in service desk scenarios.

### Features

1. **Kiosk Mode**:
    - Opens a fullscreen window with a custom URL specified in the Tauri configuration file (`tauri.conf.json`).
    - Always on top, disables window resizing and closing, creating a locked-down experience typical of kiosk applications.

2. **Windows Key Blocking**:
    - Intercepts and blocks the `Windows` key on keyboards (`VK_LWIN` and `VK_RWIN`) to prevent access to the system taskbar or shortcuts.

3. **Autostart on Boot**:
    - Supports autostart functionality with system boot using the [tauri-plugin-autostart](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/autostart) plugin.
    - Configured for macOS and Windows to ensure the application is automatically launched.

4. **Shell Integration**:
    - Uses the [tauri-plugin-shell](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/shell) for secure external process handling.

5. **Cross-Platform**:
    - Targets multiple platforms: Windows, macOS, and Linux using Tauri's lightweight and highly customizable framework.

6. **Customizable Build Configurations**:
    - Modify application settings, like URL, window dimensions, fullscreen behavior, and security policies, via the `tauri.conf.json` file.

---

### Installation and Building

Follow the steps below to clone, build, and run the project:

1. **Prerequisites**:
    - Install Rust and Cargo ([instructions here](https://www.rust-lang.org/tools/install)).
    - Install Node.js and npm ([instructions here](https://nodejs.org/)).
    - [Install Tauri prerequisites](https://tauri.app/v2/guides/getting-started/prerequisites) based on your platform.

2. **Clone Repository**:
   ```bash
   git clone <repository-url>
   cd service-desk-kiosk
   ```

3. **Install Dependencies**:
   Install Node.js dependencies required for the front end (if present):
   ```bash
   npm install
   ```

4. **Build Application**:
   Build the Tauri application by running:
   ```bash
   cargo tauri build
   ```

5. **Run Application**:
   For development:
   ```bash
   cargo tauri dev
   ```

---

### Project Structure

- **`src/main.rs`**:
  Entry point of the application. It initializes the Tauri runtime, spawns the Windows key hook thread, and sets up plugins like autostart and shell.

- **`src/windows_api.rs`**:
  Handles custom Windows-specific features like blocking specific key inputs (`Windows` key) using the WinAPI.

- **`Cargo.toml`**:
  Specifies project metadata, dependencies, and features (e.g., tauri, winapi).

- **`build.rs`**:
  Generates necessary files required for the Tauri application build process via `tauri-build`.

- **`tauri.conf.json`**:
  Central configuration for the Tauri application, including window properties, app metadata, and bundling settings.

---

### Configuration

**Tauri Configurations:**  
The behavior of the application can be changed through the `tauri.conf.json` file, e.g.:

- **URL to Load**:
  ```json
  "url": "https://fm.mardens.com/returns/"
  ```
  Change this URL to point to the website/application you'd like to display in the kiosk.

- **Window Behavior**:
  ```json
  "fullscreen": true,
  "alwaysOnTop": true,
  "width": 800,
  "height": 600
  ```
  Modify these settings to adjust window size and behaviors.

- **Icons**:
  Define custom icons for the application in the `bundle` section.

---

### How It Works

1. **Blocking Windows Key**:  
   The `windows_api::block_windows_key()` function is executed in a separate thread via `std::thread::spawn`. It uses a low-level keyboard hook through the WinAPI to intercept and block the `Windows` key.

2. **Autostart Plugin**:  
   The autostart functionality is managed via the `tauri-plugin-autostart` plugin, allowing seamless app registration to start on system boot.

3. **Shell Commands**:  
   Securely handle external commands using the `tauri-plugin-shell`.

---

### Dependencies

- **[Tauri](https://tauri.app/):** Lightweight, secure, and flexible framework for building cross-platform desktop applications.
- **[tauri-plugin-autostart](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/autostart):** Used for setting up autostart functionality.
- **[tauri-plugin-shell](https://github.com/tauri-apps/plugins-workspace/tree/v2/plugins/shell):** Shell command handling.
- **[winapi](https://docs.rs/winapi):** Native Windows API access for implementing key blocking and other low-level features.
- **[serde](https://serde.rs/) / [serde_json](https://docs.rs/serde_json):** For handling data structures and JSON serialization.

---

### Contribution

1. Fork the repository.
2. Create a feature or fix branch: `git checkout -b my-feature`.
3. Commit changes: `git commit -m "Add my feature"`.
4. Push changes: `git push origin my-feature`.
5. Submit a pull request.

---

### License

This project is licensed under the MIT License. See the LICENSE file for more details.

---

### Notes

- For Windows-specific customizations (like the Windows key blocker), the `winapi` library is used, meaning these features will not apply to other platforms.
- Always rebuild after changing `tauri.conf.json` for the changes to take effect.
