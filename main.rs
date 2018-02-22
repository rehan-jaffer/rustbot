use std::net::TcpStream;

struct Connection {
  server_list: &'static str,
  nick: &'static str,
  stream: Option<TcpStream>
}

impl Connection {
  fn new(self) -> Connection {
    return self;
  }
  fn connect(&mut self) -> bool {
    let servers = self.server_list.split("\n");
    for server_line in servers {
      if let Ok(stream) = TcpStream::connect(server_line) {
        self.stream = Some(stream);
        println!("Connected to the server!");
        return true;
      } else {
        println!("Couldn't connect to server...");
      }
    }
    return false;
  }
}

fn main() {

  let mut connection = Connection { server_list: "irc.efnet.org:6667\nirc.efnet.net:6667", nick: "dasbawt", stream: None };
  connection.connect();
}

