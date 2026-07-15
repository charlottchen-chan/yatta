use std::process::Command;

#[tauri::command]
pub fn set_nosave_mode(enabled: bool) -> Result<(), String> {
    println!("nosave status: {}", enabled);

    if enabled {
        rockstar_games_save_ip_block_enabled();
    } else {
        rockstar_games_save_ip_block_disabled();
    }

    Ok(())
}

fn rockstar_games_save_ip_block_enabled() {
    let output = std::process::Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "add",
            "rule",
            "name=custom_block_for_nosave",
            "dir=out",
            "action=block",
            "remoteip=192.81.241.171",
        ])
        .output()
        .expect("couldnot start netsh");

    if output.status.success() {
        println!("no save activ rule created");
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        let out_msg = String::from_utf8_lossy(&output.stdout);
        println!("error on creating the rule:\nStderr: {}\nStdout: {}", err_msg, out_msg);
    }
        println!("no-save active");
        // Ok(())
}

fn rockstar_games_save_ip_block_disabled() {
    let output = std::process::Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "delete",
            "rule",
            "name=custom_block_for_nosave",
        ])
        .output()
        // .map_err(|e| e.to_string());
        .expect("couldnot start netsh");

    if output.status.success() {
        println!("no save activ rule created");
    } else {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        let out_msg = String::from_utf8_lossy(&output.stdout);
        println!("error on creating the rule:\nStderr: {}\nStdout: {}", err_msg, out_msg);
    }
        // Ok(())
}

