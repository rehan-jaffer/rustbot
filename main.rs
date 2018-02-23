use std::io::Read;
use std::net::TcpStream;
use std::io::{BufReader,BufWriter};
use std::io::BufRead;
use std::io::Write;

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
    let mut writer = BufWriter::new(&self.stream);
    let mut line = String::new();
    println!("connected!");
    loop {
      let mut buf = vec![];
      let msg = reader.read_until(b'\n', &mut buf);
      let str = String::from_utf8(buf).unwrap();
      let v: Vec<&str> = str.matches("Checking Ident").collect();
      if v.len() == 1 {
        println!("sending authorization information..");
        writer.write(b"USER foo . . :real name\r\n\r\n");
        writer.write(b"NICK bar\r\n\r\n");
      }
      println!("{}", str);
    }
  }
}

fn main() {

  let mut connection = NullConnection { server_list: "irc.servercentral.net:6667\nirc.efnet.net:6667", nick: "dasbawt" };
  let mut connected = connection.connect().unwrap();
  connected.message_loop();

}

