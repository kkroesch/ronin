use anyhow::Result;
use clap::{Parser, Subcommand};

mod run; // ← Du wirst dieses Modul selbst schreiben
// mod build; // ← Optional: Wenn du später `ronin build` willst
// mod exec; // ← Optional: Wenn du später `ronin exec` willst

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a container
    Run {
        #[arg(short, long)]
        root: String,

        #[arg(short, long)]
        mount: Vec<String>,

        #[arg(short, long)]
        port: Vec<String>,

        #[arg(short, long)]
        user: Option<String>,

        #[arg(trailing_var_arg = true)]
        command: Vec<String>,
    },
    // Optional: Wenn du später `ronin build` willst
    // Build {
    //     #[arg(short, long)]
    //     output: String,
    //     #[arg(short, long)]
    //     dockerfile: Option<String>,
    // },

    // Optional: Wenn du später `ronin exec` willst
    // Exec {
    //     #[arg(short, long)]
    //     container: String,
    //     #[arg(trailing_var_arg = true)]
    //     command: Vec<String>,
    // },
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            root,
            mount,
            port,
            user,
            command,
        } => {
            run::run_container(root, mount, port, user, command)?;
        } // Optional: Wenn du später `ronin build` willst
          // Commands::Build { output, dockerfile } => {
          //     build::build_image(output, dockerfile)?;
          // }

          // Optional: Wenn du später `ronin exec` willst
          // Commands::Exec { container, command } => {
          //     exec::exec_in_container(container, command)?;
          // }
    }

    Ok(())
}
