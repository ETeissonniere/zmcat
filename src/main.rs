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
    #[clap(short, long, default_value = "tcp://localhost:5555")]
    frontend: String,
}

#[derive(Args)]
struct SubArgs {
    /// Specify the host and port to subscribe messages from in URL format.
    #[clap(short, long, default_value = "tcp://localhost:6666")]
    backend: String,
}

fn main() {
    let cli = Cli::parse();
    let context = zmq::Context::new();

    match &cli.command {
        Commands::Proxy(args) => {
            let frontend = context.socket(zmq::XSUB).unwrap();
            let backend = context.socket(zmq::XPUB).unwrap();

            frontend
                .bind(&args.frontend)
                .expect("failed binding frontend");
            backend.bind(&args.backend).expect("failed binding backend");

            zmq::proxy(&frontend, &backend).expect("failed to proxy");
        }
        Commands::Pub(args) => {
            let frontend = context.socket(zmq::PUB).unwrap();

            frontend
                .connect(&args.frontend)
                .expect("failed connecting frontend");

            let stdin = std::io::stdin();
            let mut line = String::new();

            loop {
                line.clear();
                stdin.read_line(&mut line).expect("failed reading line");

                frontend
                    .send(&line.as_bytes(), 0)
                    .expect("failed sending message");
            }
        }
        Commands::Sub(args) => {
            let backend = context.socket(zmq::SUB).unwrap();

            backend
                .connect(&args.backend)
                .expect("failed connecting backend");
            backend.set_subscribe(b"").expect("failed to subscribe");

            loop {
                let msg = backend
                    .recv_string(0)
                    .expect("failed receiving message")
                    .unwrap();

                println!("{}", msg);
            }
        }
    }
}
