use std::io::Read;

pub mod ses_crypto;

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

macro_rules! INFO {
    ($($e:expr),+ ) => {
        {
            println!($($e),*);
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

    let mut pubkeys: Vec<ses_crypto::PublicKey> = Vec::new();
    for cert_filename in args {
        INFO!("certfile={cert_filename}");
        let result: std::io::Result<ses_crypto::PublicKey> = ses_crypto::load_public_key(&cert_filename);
        if let Ok(pubkey) = result {
            if pubkey.key.len() != 0 {
                pubkeys.push(pubkey);
            } // else this certificate should be ignored (bad key usage)
        } else if let Err(err) = result {
            INFO!("Cannot load certificate '{cert_filename}': {err}");
        }
    }

    INFO!("Listening on port {port}");
    let listen_addr = std::net::SocketAddr::from(([127, 0, 0, 1], port));
    let listen_result = std::net::TcpListener::bind(listen_addr);
    let listener: std::net::TcpListener = match listen_result {
        Ok(listener) => listener,
        Err(err) => FATAL!("Error listening: {err}"),
    };

    mainloop(listener, pubkeys);
}

fn mainloop(listener: std::net::TcpListener, pubkeys: Vec<ses_crypto::PublicKey>) {
    let mut client_identifier: u32 = 0;
    for conn in listener.incoming() {
        match conn {
            Ok(conn) => {
                let pubkeys2 = pubkeys.clone();
                std::thread::spawn(move || {
                    handle_connection(conn, client_identifier, pubkeys2);
                });
            }
            Err(err) => eprintln!("Error accepting conn: {err}"),
        };
        client_identifier += 1;
    }
}

fn handle_connection(mut conn: std::net::TcpStream, client_identifier: u32, pubkeys: Vec<ses_crypto::PublicKey>) {
    INFO!("{client_identifier}: new client connected");

    let mut buffer: [u8; 10] = [0; 10];
    let mut bytes_received: Vec<u8> = Vec::new();

    loop {
        let result: std::io::Result<usize> = conn.read(&mut buffer);
        let n: usize = match result {
            Ok(n) => n,
            Err(err) => {
                INFO!("{client_identifier}: read error: {err}");
                return;
            }
        };
        if n == 0 {
            break;
        }
        let chunk_received: &[u8] = &buffer[..n];
        INFO!("{client_identifier}: got chunk: {chunk_received:?}");
        // Concatenate with bytes previously received
        bytes_received.append(&mut chunk_received.to_vec());
    }
    INFO!("{client_identifier}: number of bytes received: {}", bytes_received.len());
    if bytes_received.len() == 0 {
        // No byte received
    } else if bytes_received == "shutdown\n".as_bytes() {
        INFO!("{client_identifier}: shutdown requested");
        std::process::exit(0);
    } else {
        // authenticate_script(bytes_received, pubkeys)
        // execute_script(bytes_received, client_identifier)
    }
}
