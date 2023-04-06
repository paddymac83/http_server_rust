use std::net::TcpListener;
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop { // infinite loop

            match listener.accept() {
                Ok((mut stream, _)) => {   // "_" so that we dont use the client address
                let mut buffer = [0; 1024];  // 1024 u8's - as the ::Read train has a buffer[u8] argument
                    match stream.read(&mut buffer)  {
                        Ok(_) => {
                            println!("Received a request {}", String::from_utf8_lossy(&buffer));  // wont fail, prints the text from buffer 
                        },  // no. bytes read from buffer
                        Err(e) => {
                            println!("Failed to read buffer {}", e)
                        }
                    }// stream is returned from listerner and copied to initialised buffer
                },
                Err(e) => {
                    println!("Err {}", e);  // error object

                }

            }
        }
    }
}