use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    // Establish a TCP connection to localhost on port 22
    let tcp = TcpStream::connect("localhost:22").unwrap();

    // Initialize an SSH session and perform the handshake
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    // Load the private key from a file
    let private_key_path = "your_private_key_file_path";
    let private_key = std::fs::read_to_string(private_key_path).unwrap();

    // Authenticate with the remote SSH server using the private key
    sess.userauth_pubkey_memory("username", None, &private_key, None)
        .unwrap();

    // Open a channel to execute commands on the remote host
    let mut channel = sess.channel_session().unwrap();
    channel.exec("ifconfig").unwrap();

    // Read the output of the command from the channel
    let mut buffer = String::new();
    channel.read_to_string(&mut buffer).unwrap();
    println!("{}", buffer);

    // Close the channel and session
    channel.close().unwrap();
    sess.disconnect(None, "none", None).unwrap();
}
