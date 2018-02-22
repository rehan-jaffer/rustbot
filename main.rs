use std::io::Read;
use std::net::TcpStream;
use std::io::{BufReader,BufWriter};
use std::io::BufRead;

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
    let mut reader = BufReader::new(&self.stream);
    let mut line = String::new();
    println!("connected!");
    loop {
      let result = reader.read_line(&mut line);
      println!("{}", line[(line.len()-result)];
      println!("{} {}", line, line.len());
    }
  }
}

fn main() {

  let mut connection = NullConnection { server_list: "irc.servercentral.net:6667\nirc.efnet.net:6667", nick: "dasbawt" };
  let mut connected = connection.connect().unwrap();
  connected.message_loop();

}

