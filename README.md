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
| A5 CPH1909 32GB | Oppo  | ✅ - PASS |

---

## How To Use

1. Download and connect your phone via ADB. Refer to our detailed [Guide to Download, Open ADB on Windows, and Check Device Connection](#guide-to-download-open-adb-on-windows-and-check-device-connection).
2. Download `pkgs.zip` and `Super-Bloatware-Remove-vX.X.exe` from the latest release. You can also download the entire repository, which contains the `pkgs` folder and the main application.
3. Run `Super-Bloatware-Remove-vX.X.exe`.
4. When prompted for the **pkgs path**, provide the path to your `pkgs.txt` file.
5. When prompted for the **adb path**, paste the full path to your ADB folder (e.g., `C:\platform-tools`).

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
  com.google.android.youtube  // YouTube app
  com.facebook.appmanager     // Facebook App Manager
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

### Step 5: Run Super-Bloatware-Remove-vX.X.exe

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

Made by [Junaid](https://abujuni.dev)
