use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a dual binding pub/sub proxy.
    Proxy(ProxyArgs),

    /// Connect to a proxy's frontend and publish messages
    /// as typed in by the user.
    Pub(PubArgs),

    /// Connect to a proxy's backend and stream messages.
    Sub(SubArgs),
}

#[derive(Args)]
struct ProxyArgs {
    /// Specify the host and port to receive messages in URL format.
    #[clap(short, long, default_value = "tcp://*:5555")]
    frontend: String,

    /// Specify the host and port to forward messages in URL format.
    #[clap(short, long, default_value = "tcp://*:6666")]
    backend: String,
}

#[derive(Args)]
struct PubArgs {
    /// Specify the host and port to publish messages to in URL format.
    #[clap(short, long, default_value = "tcp://*:5555")]
    frontend: String,
}

#[derive(Args)]
struct SubArgs {
    /// Specify the host and port to subscribe messages from in URL format.
    #[clap(short, long, default_value = "tcp://*:6666")]
    backend: String,
}

fn main() {
    let cli = Cli::parse();
    let context = zmq::Context::new();

    match &cli.command {
        Commands::Proxy(args) => {
            let frontend = context.socket(zmq::SUB).unwrap();
            let backend = context.socket(zmq::PUB).unwrap();

            frontend
                .bind(&args.frontend)
                .expect("failed binding frontend");
            backend.bind(&args.backend).expect("failed binding backend");

            zmq::proxy(&frontend, &backend).expect("failed to proxy");
        }
        _ => todo!(),
    }
}
