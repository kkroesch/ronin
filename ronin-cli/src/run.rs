use anyhow::Result;

pub fn run_container(
    root: String,
    mount: Vec<String>,
    port: Vec<String>,
    user: Option<String>,
    command: Vec<String>,
) -> Result<()> {
    println!("Running container with root: {}", root);
    println!("Mounts: {:?}", mount);
    println!("Ports: {:?}", port);
    println!("User: {:?}", user);
    println!("Command: {:?}", command);

    // ← Hier kommt deine Logik
    Ok(())
}
