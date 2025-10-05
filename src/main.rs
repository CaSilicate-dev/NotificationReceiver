use serde::Deserialize;
use std::io::Read;
use std::net::TcpListener;
use std::process::Command;
#[derive(Debug, Deserialize, Default)]
struct Notification {
    title: String,
    message: String,
    urgency: String,
}
fn main() {
    let listener = match TcpListener::bind("0.0.0.0:11451") {
        Ok(r) => {
            println!("Notification Reveiver listening on 0.0.0.0:11451");
            r
        },
        Err(e) => {
            eprintln!("Failed to bind port: {}", e);
            std::process::exit(1);
        }
    };
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = Vec::new();
                match stream.read_to_end(&mut buffer) {
                    Ok(_) => match serde_json::from_slice::<Notification>(&buffer) {
                        Ok(notif) => {
                            let mut cmd = Command::new("notify-send");
                            cmd.arg("-a")
                                .arg("Notification")
                                .arg(&notif.title)
                                .arg(&notif.message)
                                .arg("-u")
                                .arg(&notif.urgency);
                            match cmd.spawn() {
                                Ok(_) => {}
                                Err(e) => {
                                    eprintln!("{}", e);
                                }
                            };
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
