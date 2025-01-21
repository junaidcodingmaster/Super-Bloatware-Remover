# Super Bloatware Remover

Effortlessly remove up to 98% of bloatware apps from your Android device without requiring root access. Developed in [Rust](https://rust-lang.org) for maximum efficiency and reliability.

---

## ⚠️ WARNING

This tool is highly powerful, even without root access. Only remove apps that you are absolutely certain are bloatware. Deleting essential system apps could cause your device to malfunction.

---

## Index

1. [Supported Devices](#supported-devices)
2. [Supported Platforms](#supported-platforms)
   - [Desktop Platforms](#desktop-platforms)
   - [Mobile Platforms](#mobile-platforms-via-termux-with-wireless-debugging)
3. [How to Use](#how-to-use)
   - [For Windows](#for-windows)
   - [For Linux](#for-linux)
4. [Important Notes](#important-notes)
5. [Customizing the `pkgs.txt` File](#customizing-the-pkgstxt-file)
6. [Guide to Download, Open ADB on Windows, and Check Device Connection](#guide-to-download-open-adb-on-windows-and-check-device-connection)
7. [Troubleshooting Tips](#troubleshooting-tips)
8. [Building the App](#building-the-app)

---

## Supported Devices

The tool has been tested on the following devices:

| Device          | Brand | Status    |
| --------------- | ----- | --------- |
| Y16-64GB        | Vivo  | ✅ - PASS |
| Y16-32GB        | Vivo  | ✅ - PASS |
| A5 CPH1909 32GB | Oppo  | ❌ - FAIL |

---

## Supported Platforms

### Desktop Platforms

| OS      | Status |
| ------- | ------ |
| Windows | ✅     |
| Linux   | ✅     |
| macOS   | ❌     |

### Mobile Platforms (via Termux with Wireless Debugging)

| OS          | Status |
| ----------- | ------ |
| Android 10+ | ✅     |
| iOS         | ✅     |

---

## How to Use

### For Windows

1. **Download and Set Up ADB**:
   Refer to our detailed [Guide to Download, Open ADB on Windows, and Check Device Connection](#guide-to-download-open-adb-on-windows-and-check-device-connection).

2. **Download the Tool**:
   Download `Super-Bloatware-Remover-vX.X.zip` from the latest release. Alternatively, download the entire repository, which includes the `pkgs` folder and the main application.

3. **Run the Tool**:
   - Execute `Super-Bloatware-Remover-vX.X.exe`.
   - Provide required paths when prompted:
     - **Pkgs Path**: Path to your `pkgs.txt` file.
     - **ADB Path**: Full path to your ADB folder (e.g., `C:\platform-tools`) or simply `adb` if it's in your system's `PATH` variable.

### For Linux

1. **Download and Set Up ADB**:
   - Install ADB (Android Debug Bridge) on your system.
   - Connect your Android device via USB cable and enable USB debugging on the device.

2. **Download the Tool**:
   - Download the `Super-Bloatware-Remover-vX.X` binary and the `pkgs.zip` file from the [releases section](https://github.com/junaidcodingmaster/Super-Bloatware-Remover/releases).
   - Extract the `pkgs` folder.

3. **Run the Tool**:
   - Navigate to the directory containing the `pkgs` folder.
   - Make the binary executable:
     ```bash
     chmod +x ./Super-Bloatware-Remover-vX.X
     ```
   - Execute the binary:
     ```bash
     ./Super-Bloatware-Remover-vX.X
     ```

4. **Provide Required Paths**:
   - **Pkgs Path**: Path to the `pkgs.txt` file (e.g., `./pkgs/vivo/pkgs.txt`).
   - **ADB Path**: Enter `adb` if it's in your system's `PATH`, or provide the full path to the ADB executable.

---

## Important Notes

- **`pkgs.txt`**: This file lists the package names of apps considered bloatware. If unsure, use the default `pkgs.txt` provided.
- **Samsung Devices**: Typically have fewer bloatware apps. A Samsung-specific list is not included.

---

## Customizing the `pkgs.txt` File

The `pkgs.txt` file is customizable, allowing you to add or remove package names as needed. Each line should contain the package name of an app to be removed.

### Example Format

```plaintext
com.google.android.youtube
com.facebook.appmanager
```

### Finding Package Names

1. Use the following ADB command to list all installed packages:
   ```bash
   adb shell pm list packages
   ```
2. Add the desired package names to your `pkgs.txt` file.

---

## Guide to Download, Open ADB on Windows, and Check Device Connection

### Step 1: Download ADB and Fastboot

1. Visit the official [Android Developers SDK Platform Tools](https://developer.android.com/studio/releases/platform-tools).
2. Download the **SDK Platform Tools** for Windows.
3. Extract the ZIP file to a convenient location, such as `C:\platform-tools`.

### Step 2: Set Up ADB

1. Open the folder where you extracted **platform-tools**.
2. Hold **Shift**, right-click inside the folder, and select **"Open PowerShell window here"** or **"Open Command Window here"**.
3. Type `adb version` and press **Enter** to verify ADB installation.

### Step 3: Enable USB Debugging on Your Android Device

1. Go to **Settings > About phone**.
2. Tap **Build number** 7 times to enable **Developer options**.
3. Go back to **Settings**, locate **Developer options**, and enable **USB debugging**.

### Step 4: Connect Your Device

1. Connect your Android device via USB cable.
2. Type `adb devices` in the command window and press **Enter**.
3. Accept the USB debugging prompt on your device if it appears.
4. Your device should appear in the list with its serial number.

### Step 5: Run the Tool

1. Execute `Super-Bloatware-Remover-vX.X.exe`.
2. Provide the required paths when prompted.

---

## Troubleshooting Tips

- Ensure your device remains connected and recognized by ADB.
- If `adb devices` doesn’t list your device:
  - Reconnect the USB cable.
  - Update USB drivers.
  - Toggle USB debugging off and on in your device settings.

---

## Building the App

If you wish to build the app on your computer due to compatibility issues, follow these steps:

1. Clone the repository and navigate to the directory:
   ```bash
   git clone https://github.com/junaidcodingmaster/Super-Bloatware-Remover.git
   cd Super-Bloatware-Remover
   ```

2. Install requirements and build the app:
   ```bash
   ./build.sh --install
   ./build.sh --build
   ```
   Refer to the [Build Script Documentation](./BUILD.md) for detailed instructions.

---

**Made by [Junaid](https://abujuni.dev)**
