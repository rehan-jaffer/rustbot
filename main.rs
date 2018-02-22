use std::io::Read;
use std::net::TcpStream;

struct NullConnection {
  server_list: &'static str,
  nick: &'static str,
}

struct Connection {
  nick: &'static str,
  stream: TcpStream
}

impl NullConnection {
  fn connect(&mut self) -> Option<Connection> {
    let servers = self.server_list.split("\n");
    for server_line in servers {
      println!("Trying to connect to {}", server_line);
      if let Ok(stream) = TcpStream::connect(server_line) {
        return Some(Connection { stream: stream, nick: self.nick })
      } else {
        println!("Couldn't connect to server...");
      }
    }
    return None;
  }
}

impl Connection {
  fn message_loop(&mut self) {
    let mut line = String::new();
    println!("connected!");
    loop {
      println!("waiting on data!");
      let result = self.stream.read_to_string(&mut line);
      match result {
        Ok(d) => println!("{}\n{}\n", d, line),
        Err(e) => println!("error reading socket!")
      }
    }
  }
}

fn main() {

  let mut connection = NullConnection { server_list: "irc.choopa.net:6667\nirc.efnet.net:6667", nick: "dasbawt" };
  let mut connected = connection.connect().unwrap();
  connected.message_loop();

}

