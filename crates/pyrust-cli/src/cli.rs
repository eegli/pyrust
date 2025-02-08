use pyrust_internal::num_cpus_available;

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    #[clap(name = "greet")]
    Greet { name: Option<String> },
    #[clap(name = "num-cpus")]
    NumCPUs,
}

pub fn run(args: Option<&Vec<String>>) {
    use clap::Parser;

    let cli = match args {
        Some(args) => Cli::parse_from(args),
        None => Cli::parse(),
    };

    match cli.command {
        Subcommand::Greet { name } => {
            println!("Hello, {}!", name.unwrap_or_else(|| "world".to_string()));
        }
        Subcommand::NumCPUs => {
            println!("{}", num_cpus_available());
        }
    };
}
