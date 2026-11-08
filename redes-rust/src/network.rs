use std::net::UdpSocket;
use std::io;

pub struct NetworkManager {
    socket: Option<UdpSocket>,
}

impl NetworkManager {
    pub fn new() -> Self {
        Self { socket: None }
    }

    pub fn bind_socket(&mut self, port: u16) -> Result<String, io::Error> {
        let addr = format!("127.0.0.1:{}", port);
        match UdpSocket::bind(&addr) {
            Ok(sock) => {
                sock.set_nonblocking(true)?;
                self.socket = Some(sock);
                Ok(format!("[NET] Socket UDP vinculado y escuchando en {}.", addr))
            }
            Err(e) => Err(e),
        }
    }

    pub fn is_active(&self) -> bool {
        self.socket.is_some()
    }
}