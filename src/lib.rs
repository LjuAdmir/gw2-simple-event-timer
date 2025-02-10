use nexus::{
    paths::get_addon_dir,
};

use std::fs;

nexus::export! {
    name: "GW2 Simple Event Timer",
    signature: -0x8008,
    load: load,
    unload: unload,
    log_filter: "debug",
}

fn load() {
    let _addon_dir = if let Some(dir) = get_addon_dir("GW2SimpleEventTimer") {
        dir
    } else {
        log::error!("Failed to get addon directory");
        return;
    };

    // Create a directory for the addon
    if let Err(_) = fs::create_dir_all(&_addon_dir) {
        log::error!("Failed to create addon directory");
        return;
    }
}

fn unload() {
}