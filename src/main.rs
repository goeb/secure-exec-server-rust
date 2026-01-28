
const USAGE: &str = "
usage: ses TCP-PORT CERTIFICATE ...

Start a TCP server where clients can submit their scripts, that get
authenticated and executed.

Arguments:
  CERTIFICATE  X509 certificate whose public key is used for authentication.
               It must be in PEM encoding.
               It must carry the x509v3 extension KeyUsage 'digitalSignature'.
               If several certificates are specified, the authentication
               will succeed if at least 1 certificate verifies the signature
  TCP-PORT     Listening port
";

fn usage() {
    println!("{USAGE}");
    std::process::exit(1);
}

// Macro FATAL: print an error message and exit with exit code 1
// Eg: FATAL!("Error x={x}, y={y}", x, y")
macro_rules! FATAL {
    ($($e:expr),+ ) => {
        {
            eprint!("FATAL: ");
            eprintln!($($e),*);
            std::process::exit(1);
        }
    };
}


fn main() {

    let mut args = std::env::args();
    if args.len() < 3 {
        usage();
    }

    let _ = args.next(); // skip the program name
    let port_str: String = args.next().unwrap();
    let port: u16 = match u16::from_str_radix(&port_str, 10)  {
        Ok(port) => port,
        Err(err) => FATAL!("Invalid argument TCP-PORT: {port_str} ({err})"),
    };

    for certfile in args {
        // TODO load public key from certificate
        println!("certfile={certfile}");
    }
    let pubkeys = (); // TODO

    println!("Listening on port {port}");
    let listen_addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listen_result = std::net::TcpListener::bind(listen_addr);
    let listener: std::net::TcpListener = match listen_result {
        Ok(listener) => listener,
        Err(err) => FATAL!("Error listening: {err}"),
    };

    mainloop(listener, pubkeys);
}

fn mainloop(listener: std::net::TcpListener, pubkeys: ()) {
    let mut client_identifier: u32 = 0;
    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                handle_connection(conn, client_identifier, pubkeys);
            }
            Err(err) => eprintln!("Error accepting conn: {err}"),
        };
        client_identifier += 1;
    }
}

fn handle_connection(conn: std::net::TcpStream, client_identifier: u32, pubkeys: ()) {
    FATAL!("handle_connection not implemented");
}
