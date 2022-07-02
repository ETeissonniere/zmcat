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

    /// If set to true, the proxy will spawn a new thread to capture
    /// and log all messages going through it.
    #[clap(short, long)]
    capture: bool,
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

fn capture_logger(context: &zmq::Context) {
    let receiver = context.socket(zmq::PAIR).unwrap();
    receiver
        .connect("inproc://capture")
        .expect("failed to connect to capture socket");

    loop {
        let msg = receiver.recv_msg(0).unwrap();
        println!("{:?}", msg);
    }
}

fn main() {
    let cli = Cli::parse();
    let context = zmq::Context::new();

    match &cli.command {
        Commands::Proxy(args) => {
            let mut frontend = context.socket(zmq::XSUB).unwrap();
            let mut backend = context.socket(zmq::XPUB).unwrap();

            frontend
                .bind(&args.frontend)
                .expect("failed binding frontend");
            backend.bind(&args.backend).expect("failed binding backend");

            if args.capture {
                let mut capture_receiver = context.socket(zmq::PAIR).unwrap();
                capture_receiver
                    .bind("inproc://capture")
                    .expect("failed to bind capture receiver socket");

                std::thread::spawn(move || capture_logger(&context));

                zmq::proxy_with_capture(&mut frontend, &mut backend, &mut capture_receiver)
                    .expect("failed to proxy");
            } else {
                zmq::proxy(&frontend, &backend).expect("failed to proxy");
            }
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
