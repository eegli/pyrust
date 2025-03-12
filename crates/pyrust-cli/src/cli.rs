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
    /// Greet someone.
    Greet {
        #[clap(long, short, default_value = "world")]
        name: String,
    },

    /// Print the number of (logical) CPU cores available on the system.
    #[clap(name = "num-cpus")]
    NumCPUs {
        #[clap(long, short, default_value = "false")]
        /// Only count physical CPU cores.
        physical: bool,
    },

    /// Estimate the value of pi using a Monte Carlo method.
    EstimatePi {
        /// Number of samples to use.
        #[clap(long, short)]
        num_samples: usize,
        #[clap(long, short = 't')]
        /// Number of threads to use. If not provided, the number of CPUs available on the system will be used.
        num_threads: Option<usize>,
    },
}

pub fn run(args: Option<&Vec<String>>) {
    use clap::Parser;

    let cli = match args {
        Some(args) => Cli::parse_from(args),
        None => Cli::parse(),
    };

    match cli.command {
        Subcommand::Greet { name } => {
            println!("Hello, {name}!",);
        }
        Subcommand::NumCPUs { physical } => {
            println!("{}", num_cpus_available(physical));
        }
        Subcommand::EstimatePi {
            num_samples,
            num_threads,
        } => {
            let pi_estimated = pyrust_internal::pi::monte_carlo_pi(num_samples, num_threads);
            let pi_actual = std::f32::consts::PI;
            let error = (pi_estimated - pi_actual).abs();
            println!("🍰 Estimated Pi to be {pi_estimated}. That's an error of {error}.");
        }
    };
}
