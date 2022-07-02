use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Specify the host and port to receive messages in URL format.
    #[clap(short, long, default_value = "tcp://*:5555")]
    frontend: String,

    /// Specify the host and port to forward messages in URL format.
    #[clap(short, long, default_value = "tcp://*:6666")]
    backend: String,
}

fn main() {
    let cli = Cli::parse();

    let context = zmq::Context::new();
    let frontend = context.socket(zmq::SUB).unwrap();
    let backend = context.socket(zmq::PUB).unwrap();

    frontend
        .bind(&cli.frontend)
        .expect("failed binding frontend");
    backend.bind(&cli.backend).expect("failed binding backend");

    zmq::proxy(&frontend, &backend).expect("failed to proxy");
}
