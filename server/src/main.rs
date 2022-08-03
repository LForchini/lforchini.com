use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Flag for whether running in production mode or not.
    /// This will enable HTTPS features.
    #[clap(long)]
    production: bool,

    /// Path to private key
    #[clap(short, long, value_parser, value_name = "FILE")]
    key_filename: Option<String>,

    /// Path to HTTPS certificate
    #[clap(short, long, value_parser, value_name = "FILE")]
    cert_filename: Option<String>,

    /// Port to run on
    #[clap(short, long, value_parser)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    log::info!("Initialising server");

    log::debug!("Reading arguments");
    let cli_args = Args::parse();

    if cli_args.production {
        log::debug!("Running in production mode");
        if cli_args.key_filename == None || cli_args.cert_filename == None {
            if cli_args.key_filename == None {
                log::error!("Key file was not specified");
            }
            if cli_args.cert_filename == None {
                log::error!("Cert file was not specified");
            }

            panic!("Cannot run in production mode without specifying HTTPS files");
        }
    } else {
        log::debug!("Running in developer mode");
    }
}
