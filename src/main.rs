use std::fs;
use std::io;
use std::process::Command;
use ctrlc;
use std::time::Duration;
use std::thread;

const BANNER: &str = "
███████╗██╗   ██╗██████╗ ███████╗██████╗
██╔════╝██║   ██║██╔══██╗██╔════╝██╔══██╗
███████╗██║   ██║██████╔╝█████╗  ██████╔╝
╚════██║██║   ██║██╔═══╝ ██╔══╝  ██╔══██╗
███████║╚██████╔╝██║     ███████╗██║  ██║
╚══════╝ ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═╝

██████╗ ██╗      ██████╗  █████╗ ████████╗██╗    ██╗ █████╗ ██████╗ ███████╗
██╔══██╗██║     ██╔═══██╗██╔══██╗╚══██╔══╝██║    ██║██╔══██╗██╔══██╗██╔════╝
██████╔╝██║     ██║   ██║███████║   ██║   ██║ █╗ ██║███████║██████╔╝█████╗  
██╔══██╗██║     ██║   ██║██╔══██║   ██║   ██║███╗██║██╔══██║██╔══██╗██╔══╝  
██████╔╝███████╗╚██████╔╝██║  ██║   ██║   ╚███╔███╔╝██║  ██║██║  ██║███████╗
╚═════╝ ╚══════╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝    ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝

██████╗ ███████╗███╗   ███╗ ██████╗ ██╗   ██╗███████╗██████╗
██╔══██╗██╔════╝████╗ ████║██╔═══██╗██║   ██║██╔════╝██╔══██╗
██████╔╝█████╗  ██╔████╔██║██║   ██║██║   ██║█████╗  ██████╔╝
██╔══██╗██╔══╝  ██║╚██╔╝██║██║   ██║╚██╗ ██╔╝██╔══╝  ██╔══██╗
██║  ██║███████╗██║ ╚═╝ ██║╚██████╔╝ ╚████╔╝ ███████╗██║  ██║
╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝ ╚═════╝   ╚═══╝  ╚══════╝╚═╝  ╚═╝

v1.2.3

 ~ Made By Junaid (abujuni.dev)
";

static mut ADB_IS_GLOBAL: bool = false;

fn simple_sleep(milliseconds: u64) {
    thread::sleep(Duration::from_millis(milliseconds));
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear screen");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen");
    }
}

fn ctrl_c_error_handler() {
    ctrlc::set_handler(move || {
        println!("\nExiting...");
        simple_sleep(800);
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn command_exc(mut command: String) {
    unsafe {
        if ADB_IS_GLOBAL {
            command = "adb".to_string();
        }
    }

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &command])
            .output()
            .expect("Failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .expect("Failed to execute process")
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("Command Exec > {}", command);
    if !stdout.is_empty() {
        println!("Output: {}", stdout);
    }
    if !stderr.is_empty() {
        println!("Error: {}", stderr);
    }
}

fn read_file(file_path: &str) -> Vec<String> {
    match fs::read_to_string(file_path.trim()) {
        Ok(file_data) => file_data.lines().map(str::to_string).collect(),
        Err(e) => {
            println!("ERROR: Unable to read file - {}", e);
            vec![]
        }
    }
}

fn super_delete(adb_path: &str, pkg: &str) {
    let commands = [
        format!("{} shell pm disable-user --user 0 {}", adb_path, pkg),
        format!("{} shell pm uninstall --user 0 {}", adb_path, pkg),
        format!("{} shell rm -rf /system/app/{}", adb_path, pkg),
    ];

    for command in commands {
        command_exc(command);
    }
}

fn app() {
    let mut adb_file_path = String::new();
    let mut pkgs_file_path = String::new();

    println!();
    if cfg!(target_os = "windows") {
        println!("Enter adb.exe path (or 'adb' if it is in PATH):");
    } else {
        println!("Enter adb binary path (or 'adb' if it is in PATH):");
    }

    io::stdin()
        .read_line(&mut adb_file_path)
        .expect("ERROR: Unable to read input!");
    let adb_file_path = adb_file_path.trim().replace('"', "").to_string();

    println!();
    println!("Enter pkg.txt file full path:");
    io::stdin()
        .read_line(&mut pkgs_file_path)
        .expect("ERROR: Unable to read input!");
    let pkgs_file_path = pkgs_file_path.trim().to_string();

    let package_names = read_file(&pkgs_file_path);

    println!("LOGS :\n");
    unsafe {
        if adb_file_path == "adb" {
            ADB_IS_GLOBAL = true;
        }
    }

    for pkg in package_names {
        if !pkg.is_empty() {
            super_delete(&adb_file_path, &pkg);
        }
    }
}

fn main() {
    clear_screen();
    println!("{}", BANNER);

    ctrl_c_error_handler();
    app();
}
