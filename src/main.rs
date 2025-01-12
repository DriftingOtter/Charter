use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

mod port_utils;

fn main() {

}

fn init_remote_ssh(host: &str, username: &str) -> Option<Session> {
    // Connect to the local SSH server
    let mut session = Session::new().expect("Couldn't creat ssh session");
    let tcp = TcpStream::connect(host).expect("Couldn't wrap tcp host address");

    session.set_tcp_stream(tcp);
    session.handshake().expect("Remote ssh handshake failed");
    session.userauth_agent(username).expect("Failed ssh session usr auth");

    return Some(session);
}

fn execute_remote_comand(session: Session, command: &str) -> Option<String> {
    let mut channel = session.channel_session().unwrap();
    channel.exec(command).unwrap();

    let mut buf = String::new();
    channel.read_to_string(&mut buf).unwrap();

    println!("{}", buf);
    channel.wait_close().expect("ERROR, cound't close ssh channel");

    return Some(buf);
}
