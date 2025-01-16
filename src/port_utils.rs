use std::thread;
use std::io::{self, Read};
use ssh2::Session;
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};

pub const LOCAL_HOST: &str = "127.0.0.1";

pub fn find_openport(mut range: std::ops::Range<u16>) -> Result<u16, &'static str> {
    range
        .find(|&port| port_is_available(port))
        .ok_or("No available port found in the specified range")
}

pub fn port_is_available(port: u16) -> bool {
    TcpListener::bind((LOCAL_HOST, port)).is_ok()
}

pub fn init_ssh_session(
    local_host: &str,
    local_port: u16,
    remote_host: &str,
    username: &str,
    password: &str,
) -> Result<Session, String> {
    println!(
        "Initializing SSH connection to {}@{}:{}...",
        username, remote_host, local_port
    );

    let tcp = TcpStream::connect((local_host, local_port))
        .map_err(|e| format!("Failed to connect to local host: {}", e))?;
    tcp.set_read_timeout(Some(Duration::from_secs(60)))
        .map_err(|e| format!("Failed to set read timeout: {}", e))?;
    tcp.set_write_timeout(Some(Duration::from_secs(60)))
        .map_err(|e| format!("Failed to set write timeout: {}", e))?;

    let mut session = Session::new().expect("Failed to create SSH session");
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .map_err(|e| format!("Failed to perform SSH handshake: {}", e))?;
    session
        .userauth_password(username, password)
        .map_err(|e| format!("Authentication failed: {}", e))?;
    if !session.authenticated() {
        return Err("Authentication failed".to_string());
    }
    println!("Authenticated to the SSH server.");

    Ok(session)
}

pub fn create_ephemeral_openport(local_port: u16, lifetime: u64) -> Result<Vec<Vec<u8>>, String> {
    println!("Listening on local port {} for {} seconds...", local_port, lifetime);

    let listener = TcpListener::bind((LOCAL_HOST, local_port))
        .map_err(|e| format!("Failed to bind to port {}: {}", local_port, e))?;
    listener
        .set_nonblocking(true)
        .map_err(|e| format!("Failed to set non-blocking mode: {}", e))?;

    let mut log = Vec::new();
    let time_limit = Duration::from_secs(lifetime);
    let start_time = Instant::now();

    loop {
        if start_time.elapsed() >= time_limit {
            println!("Port {} lifetime expired. Closing listener.", local_port);
            break;
        }
        match listener.accept() {
            Ok((mut stream, addr)) => {
                println!("Connection received from: {}", addr);
                let mut buffer = vec![0; 1024];

                match stream.read(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let data = buffer[..bytes_read].to_vec();
                        log.push(data.clone());
                        println!("Received data: {:?}", data);
                    }
                    Ok(_) => println!("Connection closed by client."),
                    Err(e) => eprintln!("Error reading stream: {}", e),
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // Sleep briefly to prevent busy-waiting
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => return Err(format!("Error accepting connection: {}", e)),
        }
    }

    return Ok(log);
}

pub fn create_ephemeral_remote_openport(
    local_host: &str,
    local_port: u16,
    remote_host: &str,
    remote_port: u16,
    username: &str,
    password: &str,
    lifetime: u64,
) -> Result<Vec<Vec<u8>>, String> {
    // Connect to the remote server
    let tcp = TcpStream::connect((local_host, local_port))
        .map_err(|e| format!("Failed to connect to local host: {}", e))?;
    
    // Set the TcpStream to non-blocking
    tcp.set_nonblocking(true)
        .map_err(|e| format!("Failed to set non-blocking mode: {}", e))?;
    
    tcp.set_read_timeout(Some(Duration::from_secs(lifetime)))
        .map_err(|e| format!("Failed to set read timeout: {}", e))?;
    
    tcp.set_write_timeout(Some(Duration::from_secs(lifetime)))
        .map_err(|e| format!("Failed to set write timeout: {}", e))?;

    // Initialize SSH session
    let mut session = Session::new().expect("Failed to create SSH session");
    session.set_tcp_stream(tcp);
    session
        .handshake()
        .map_err(|e| format!("Failed to perform SSH handshake: {}", e))?;

    // Authenticate
    session
        .userauth_password(username, password)
        .map_err(|e| format!("Authentication failed: {}", e))?;
    
    if !session.authenticated() {
        return Err("Authentication failed".to_string());
    }
    println!("Authenticated to the SSH server.");

    // Create a port forwarding tunnel
    let mut channel = session
        .channel_direct_tcpip(remote_host, remote_port, Some((local_host, local_port)))
        .map_err(|e| format!("Failed to create port forwarding tunnel: {}", e))?;

    let start_time = std::time::Instant::now();
    let mut log: Vec<Vec<u8>> = Vec::new();

    // Read data until lifetime expires
    while start_time.elapsed() < Duration::from_secs(lifetime) {
        let mut buffer = [0; 1024];
        match channel.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                println!("Received data: {:?}", &buffer[..bytes_read]);
                log.push(buffer[..bytes_read].to_vec());
            }
            Ok(_) => {
                println!("No data received.");
            }
            Err(e) => {
                eprintln!("Error reading from channel: {}", e);
                break;
            }
        }
    }

    // Close the channel
    channel
        .send_eof()
        .map_err(|e| format!("Failed to send EOF to channel: {}", e))?;
    channel
        .close()
        .map_err(|e| format!("Failed to close channel: {}", e))?;
    channel
        .wait_close()
        .map_err(|e| format!("Failed to wait for channel close: {}", e))?;

    println!(
        "Remote open port {} on {}@{} lifetime reached. Port closed.",
        remote_port, username, remote_host
    );

    return Ok(log);
}

