use clap::{Parser, Subcommand};

/// GPUI component toolkit CLI
#[derive(Parser)]
#[command(name = "gpui", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a component to your project
    Add {
        /// Component name
        component: String,
        /// Output the mutation plan as JSON instead of applying
        #[arg(long)]
        plan: bool,
    },
    /// Generate a mutation plan for a component
    Plan {
        /// Component name
        component: String,
    },
    /// Apply a previously generated mutation plan
    Apply {
        /// Path to the plan JSON file
        plan_file: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { component, plan } => {
            if plan {
                println!("Planning add for component: {component}");
            } else {
                println!("Adding component: {component}");
            }
        }
        Commands::Plan { component } => {
            println!("Generating plan for component: {component}");
        }
        Commands::Apply { plan_file } => {
            println!("Applying plan from: {plan_file}");
        }
    }

    Ok(())
}
