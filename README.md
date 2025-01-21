# Super Bloatware Remover

Effortlessly remove up to 98% of bloatware apps from your Android device without needing root access. Developed in [Rust](https://rust-lang.org) for efficiency and reliability.

## ⚠️ WARNING!

This tool is very powerful, even without rooting your device. Only remove apps that you are certain are bloatware. Deleting essential system apps could lead to device malfunctions.

---

## Supported Devices

The tool has been tested on the following devices:

| Device          | Brand | Status    |
| --------------- | ----- | --------- |
| Y16-64GB        | Vivo  | ✅ - PASS |
| Y16-32GB        | Vivo  | ✅ - PASS |
| A5 CPH1909 32GB | Oppo  | ❌ - FAIL |

## Supported Platforms

| OS      | Status |
| ------- | ------ |
| WINDOWS | ✅     |
| LINUX   | ✅     |
| MACOS   | ❌     |

**Via Termux** - You need to use wireless debugging (supported for Android 10 and above):
| OS        | Status |
| --------- | ------ |
| ANDROID 10+ | ✅   |
| IOS       | ✅     |

---

## How To Use

## WINDOWS
1. Download and connect your phone via ADB. Refer to our detailed [Guide to Download, Open ADB on Windows, and Check Device Connection](#guide-to-download-open-adb-on-windows-and-check-device-connection).
2. Download `Super-Bloatware-Remover-vX.X.zip` from the latest release. You can also download the entire repository, which contains the `pkgs` folder and the main application.
3. Run `Super-Bloatware-Remove-vX.X.exe`.
4. When prompted for the **pkgs path**, provide the path to your `pkgs.txt` file.
5. When prompted for the **adb path**, paste the full path to your ADB folder (e.g., `C:\platform-tools`) or just type `adb` , if it exist in `PATH` var.

## LINUX

1. **Download and Connect Your Phone via ADB**:
   - Ensure that ADB (Android Debug Bridge) is installed on your system.
   - Connect your Android device to your computer using a USB cable and enable USB debugging on your device.

2. **Download the `Super-Bloatware-Remover` Binary**:
   - Download the `Super-Bloatware-Remover-vX.X.` binary file from the [releases section](https://github.com/junaidcodingmaster/Super-Bloatware-Remover/releases).
   - Download the `pkgs.zip` file from the [releases section](https://github.com/junaidcodingmaster/Super-Bloatware-Remover/releases).
   - Extract pkgs

3. **Run the Super Bloatware Remover**:
   - Navigate to the directory containing the extracted pkgs files.
   - Make the binary executable:
     ```bash
     chmod +x ./Super-Bloatware-Remover-vX.X
     ```
   - Execute the binary:
     ```bash
     ./Super-Bloatware-Remover-vX.X
     ```

4. **Provide Required Paths When Prompted**:
   - **Pkgs Path**: Enter the path to the `pkgs.txt` file corresponding to your phone's brand. For example:
     ```
     ./pkgs/vivo/pkgs.txt
     ```
   - **ADB Path**: If ADB is in your system's `PATH`, simply type `adb`. Otherwise, provide the full path to your ADB executable.

---

## Important Notes

- **`pkgs.txt`**: A text file listing the package names of apps considered bloatware. If you’re unsure which apps to remove, use the default `pkgs.txt` provided.
- **Samsung Devices**: Typically have fewer bloatware apps. Therefore, a comprehensive Samsung-specific list is not included.

---

## Customizing the `pkgs.txt` File

The `pkgs.txt` file is flexible, allowing you to add your own bloatware apps or create a custom list from scratch. Each line in the file should list the package name of an app you want to remove.

### How to Format Your Custom List:

- Write each app's package name on a new line.
- Example:
  ```
  com.google.android.youtube
  com.facebook.appmanager
  ```

### How to Find and Add More Bloatware Apps:

1. **Finding Package Names**: Use this ADB command to list all installed packages:
   ```
   ./adb.exe pm list packages
   ```
2. **Creating a Custom List**: If the default `pkgs.txt` file doesn’t include all the apps you consider bloatware, customize it fully to suit your needs.

---

### Guide to Download, Open ADB on Windows, and Check Device Connection

#### Step 1: Download ADB and Fastboot

1. Visit the official [Android Developers SDK Platform Tools](https://developer.android.com/studio/releases/platform-tools).
2. Download the **SDK Platform Tools** for Windows.
3. Extract the ZIP file to a convenient location, such as `C:\platform-tools`.

---

#### Step 2: Set Up ADB

1. Open the folder where you extracted **platform-tools**.
2. Hold down the **Shift** key, right-click inside the folder, and select **"Open PowerShell window here"** or **"Open Command Window here"**.
3. Type `adb version` and press **Enter** to verify that ADB is working. You should see the ADB version displayed.

---

#### Step 3: Enable USB Debugging on Your Android Device

1. Go to **Settings > About phone**.
2. Tap **Build number** 7 times to enable **Developer options**.
3. Go back to **Settings**, find **Developer options**, and enable **USB debugging**.

---

#### Step 4: Connect Your Android Device

1. Connect your Android device to your PC using a USB cable.
2. In the command window, type `adb devices` and press **Enter**.
3. Accept the USB debugging prompt on your device if it appears.
4. Your device should be listed with its serial number, indicating a successful connection.

---

### Step 5: Run Super-Bloatware-Remove -vX.X.exe

1. Locate and run **Super-Bloatware-Remove-vX.X.exe** on your computer.
2. Follow the on-screen prompts:
   - **Pkgs Path**: Enter the path to your `pkgs.txt` file.
   - **ADB Path**: Enter the full path to your ADB folder (e.g., `C:\platform-tools`).

---

### Troubleshooting Tips

- Ensure your device remains connected and is recognized by ADB throughout the process.
- If `adb devices` doesn’t list your device, try:
  - Reconnecting the USB cable.
  - Updating your USB drivers.
  - Toggling USB debugging off and back on in your device settings.

---

## Build

Do you want to build this app on your computer , due to hardware compatible issues then follow these steps:
1. Clone this repo and change dir
```
git clone https://github.com/junaidcodingmaster/Super-Bloatware-Remover.git
cd Super-Bloatware-Remover
```
2. Install requriments and Build it
```
./build.sh --install
./build.sh --build
```
Befour building read detailled documentation of build script - [Build Script Documentation](./BUILD.md)

---

Made by [Junaid](https://abujuni.dev)
