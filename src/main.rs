use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

fn main() -> std::io::Result<()> {

    let storage: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    // Initialises version but doesn't keep the lock
    {
        let mut version = storage.lock().unwrap();
        version.insert("version".to_string(), "Ken's Key-Value Store 1.0".to_string());
    } // Lock is released here

    let socket = UdpSocket::bind("0.0.0.0:8080")?;
    let mut buf = [0; 1024];

    println!("unusual-database-program running on 0.0.0.0:8080...");

    loop {
        match socket.recv_from(&mut buf) {
            Ok((_, src)) => {
                if let Ok(message) = String::from_utf8(buf.to_vec()) {
                    let message = message.trim_matches(char::from(0)).trim();

                    // This block returns the version number to the client
                    if message == "version" {
                        let store = storage.lock().unwrap();
                        if let Some(version) = store.get("version") {
                            socket.send_to(version.as_bytes(), src)?;
                        }
                    
                    } else if message.contains('=') {
                        let mut parts = message.splitn(2, '=');
                        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
                            // Checks that the message doesn't contain a version, otherwise ignores
                            if key != "version" {
                            let mut store = storage.lock().unwrap();
                            store.insert(key.to_string(), value.to_string());
                            }
                        }
                    } else {
                        let store = storage.lock().unwrap();
                            if let Some(value) = store.get(message) {
                            socket.send_to(format!("{}={}", message, value).as_bytes(), src)?;
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error receiving data: {}", e),
        }

        buf = [0; 1024];
    }
}
