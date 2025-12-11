use anyhow::Result;
use nix::mount::{mount, MsFlags};
use nix::unistd::{chroot, fork, Pid};
use std::process::Command;

fn main() -> Result<()> {
    env_logger::init();

    println!("🚀 Ronin Init started...");

    // 1. Mount /proc, /sys, /dev, /tmp
    mount_proc_sys_dev_tmp()?;

    // 2. Starte den Server-Prozess
    start_server()?;

    Ok(())
}

fn mount_proc_sys_dev_tmp() -> Result<()> {
    mount(
        Some("proc"),
        "/proc",
        Some("proc"),
        MsFlags::empty(),
        None::<&str>,
    )?;
    mount(
        Some("sysfs"),
        "/sys",
        Some("sysfs"),
        MsFlags::empty(),
        None::<&str>,
    )?;
    mount(
        Some("tmpfs"),
        "/tmp",
        Some("tmpfs"),
        MsFlags::empty(),
        None::<&str>,
    )?;
    mount(
        Some("devtmpfs"),
        "/dev",
        Some("devtmpfs"),
        MsFlags::empty(),
        None::<&str>,
    )?;

    println!("✅ /proc, /sys, /dev, /tmp mounted");
    Ok(())
}

fn start_server() -> Result<()> {
    println!("⚡ Starting server process...");

    let status = Command::new("/bin/bash").status()?;

    println!("✅ Server exited with status: {}", status);

    Ok(())
}
