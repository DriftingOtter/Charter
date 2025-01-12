use std::io::Read;
use std::net::TcpListener;
use std::time::{Duration, Instant};

const LOCAL_HOST: &str = "127.0.0.1";

fn main() {
    let buffer = create_ephemeral_openport(10);

    if !buffer.is_empty() {
        println!("Log of received data:");
        for (i, entry) in buffer.iter().enumerate() {
            match convert_buffer_to_string(entry.clone()) {
                Ok(text) => {
                    println!("#{}: {}", i + 1, text);
                }
                Err(err) => {
                    println!("#{}: {}", i + 1, err);
                }
            }
        }
    } else {
        println!("No data received during lifetime.");
    }
}

fn convert_buffer_to_string(buffer: Vec<u8>) -> Result<String, String> {
    match String::from_utf8(buffer) {
        Ok(text) => Ok(text),
        Err(_) => Err("Received non-UTF-8 data, unable to convert to string".to_string()),
    }
}

fn find_openport() -> Option<u16> {
    (8000..9000)
        .find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match TcpListener::bind((LOCAL_HOST, port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn create_ephemeral_openport(lifetime: u64) -> Vec<Vec<u8>> {
    let port = find_openport().expect("Couldn't find an open port to bind to");

    let time_limit = Duration::from_secs(lifetime);
    let start_time = Instant::now();

    let listener = TcpListener::bind((LOCAL_HOST, port)).expect("Failed to bind to port");
    if let Err(_) = listener.set_nonblocking(true) {
        eprintln!("Error encountered during setting non-blocking mode");
    }

    let mut log: Vec<Vec<u8>> = Vec::new();
    loop {
        if start_time.elapsed() >= time_limit {
            println!("Open port {} lifetime reached.", port);
            println!("Closing port...");
            break;
        }

        // Accept incoming traffic
        match listener.accept() {
            Ok((mut stream, addr)) => {
                println!("Connection received from: {}", addr);

                let mut buffer = Vec::new();
                match stream.read_to_end(&mut buffer) {
                    Ok(bytes_read) => {
                        if bytes_read == 0 {
                            println!("No data received or connection closed by client.");
                        } else {
                            println!("Received data: {:?}", buffer);
                            log.push(buffer.clone());

                            // Remove later since not all data is a string
                            if let Ok(text) = String::from_utf8(buffer.clone()) {
                                println!("Received text: {}", text);
                            } else {
                                println!("Received non-UTF-8 data, unable to convert to string");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error reading from stream: {}", e);
                    }
                }

                drop(stream);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No connection ready, sleep and try again
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                break;
            }
        }
    }

    return log;
}

