# A low footprint ZeroMQ N-to-N Pub/Sub Proxy (and some candies)
`zmcat` is an extremely simple Pub/Sub proxy to allow N-Pub to N-Sub usecases.

## Docker
Images available on both DockerHub and Github Container Registry:
- `eteissonniere/zmcat`
- `ghcr.io/eteissonniere/zmcat`

## Usage

### Proxy
Relies on ZMQ `XSUB` and `XPUB` sockets, as well as the `proxy` function from ZMQ. Will not display
anything but will effectively proxy messages from the XSUB socket to the XPUB socket.

```
zmcat-proxy 
Start a dual binding pub/sub proxy

USAGE:
    zmcat proxy [OPTIONS]

OPTIONS:
    -b, --backend <BACKEND>      Specify the host and port to forward messages in URL format
                                 [default: tcp://*:6666]
    -c, --capture                If set to true, the proxy will spawn a new thread to capture and
                                 log all messages going through it
    -f, --frontend <FRONTEND>    Specify the host and port to receive messages in URL format
                                 [default: tcp://*:5555]
    -h, --help                   Print help information
```

### Pub
Will connect to a ZMQ XPUB socket and publish messages to it. To publish messages, simply type them.

```
zmcat-pub 
Connect to a proxy's frontend and publish messages as typed in by the user

USAGE:
    zmcat pub [OPTIONS]

OPTIONS:
    -f, --frontend <FRONTEND>    Specify the host and port to publish messages to in URL format
                                 [default: tcp://localhost:5555]
    -h, --help                   Print help information
```

### Sub
Will connect to a ZMQ XSUB socket and subscribe to messages from it, will display messages received on stdout.

```
zmcat-sub 
Connect to a proxy's backend and stream messages

USAGE:
    zmcat sub [OPTIONS]

OPTIONS:
    -b, --backend <BACKEND>    Specify the host and port to subscribe messages from in URL format
                               [default: tcp://localhost:6666]
    -h, --help                 Print help information
```