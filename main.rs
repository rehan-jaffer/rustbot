use std::io::Read;
use std::net::TcpStream;
use std::io::{BufReader,BufWriter};
use std::io::BufRead;
use std::io::Write;
use std::thread;
use std::sync::Arc;
use std::sync::Mutex;

struct NullConnection {
  server_list: &'static str,
  nick: &'static str,
}

struct ActiveConnection<'a> {
  stream: &'a TcpStream,
  reader: BufReader<TcpStream>,
  writer: BufWriter<TcpStream>
}

struct IRCMessage {
  nick: &'static str,
  ident: &'static str,
  host: &'static str,
  mtype: &'static str,
  to: &'static str,
  msg: &'static str
}

struct Connection {
  nick: &'static str,
  stream: TcpStream
}

struct PingHandler<'b> {
  stream: &'b TcpStream,
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

impl<'b> PingHandler<'b> {
  fn ping_loop(&mut self) {
    loop {
 //     let mut buf = vec![];
   //   let msg = self.reader.read_until(b'\n', &mut buf);
     // let str = String::from_utf8(buf).unwrap();
      // println!("{}", str);      
    }
  }
}

impl Connection {
  fn message_loop(&self) -> Option<ActiveConnection> {
    let mut reader = BufReader::new(&self.stream);
    let mut writer = BufWriter::new(&self.stream);
    let mut line = String::new();
    println!("connected!");
    loop {
      let mut buf = vec![];
      let msg = reader.read_until(b'\n', &mut buf);
      let str = String::from_utf8(buf).unwrap();
      let ident_line: Vec<&str> = str.matches("Checking Ident").collect();
      let motd_line: Vec<&str> = str.matches("End of MOTD").collect();
      if ident_line.len() == 1 {
        println!("sending authorization information..");
        let res = writer.write_all(b"NICK mybotname\n");
        match res {
          Ok(a) => println!("sent data!"),
          Err(e) => println!("{}", e)
        }
        writer.write_all(b"USER boaty . . :real name a\r\n\r\n\r\n\n");
        match writer.flush() {
          Ok(a) => { println!("ok!") },
          Err(b) => { println!("{}", b) }
        }
      }
      if motd_line.len() == 1 {
        println!("received end of MOTD");
        return Some(ActiveConnection { stream: &self.stream, reader: BufReader::new(self.stream.try_clone().expect("failed")), writer: BufWriter::new(self.stream.try_clone().expect("failed")) });
      }

      println!("{}", str);

    }

  }
}

impl<'a> ActiveConnection<'a> {
  fn join(&mut self, channel:String) {
    println!("Joining #{}", channel);
    let join_str = format!("JOIN {}\n", channel);
    self.writer.write_all(String::from(join_str).as_bytes());
    self.writer.flush();
  }
  fn message_loop(&mut self) {
    loop {
      let mut buf = vec![];
      let msg = self.reader.read_until(b'\n', &mut buf);
      let str = String::from_utf8(buf).unwrap();
      println!("{}", str);
    }
  }  
  fn ping_handler(&self) -> PingHandler {
    return PingHandler { stream: &self.stream.clone() }
  }
}

fn main() {
  let channel_list = vec!["#bottesting", "#morebottesting"];
  let mut connection = NullConnection { server_list: "irc.servercentral.net:6667\nirc.efnet.net:6667", nick: "dasbawt" };

  let mut connected = connection.connect().unwrap();
  let mut active = connected.message_loop().unwrap();

    for chan in channel_list {
      active.join(String::from(chan));
    }

    active.message_loop();

  {

    let mut ping_handler = Arc::new(active.ping_handler());

   // thread::spawn(move || {
   //   ping_handler.ping_loop();
   //  } );

  }

}

