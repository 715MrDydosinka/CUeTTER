use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::erros::PipeError;

pub fn read_pipe() -> Result<String, PipeError> {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let mut buffer = Vec::new();
        match io::stdin().read_to_end(&mut buffer) {
            Ok(_) => {
                let _ = tx.send(buffer);
            }
            Err(_) => {
                let _ = tx.send(vec![]);
            }
        }
    });
    
    let timeout_duration = Duration::from_secs(1);
    let start = std::time::Instant::now();
    
    let buffer = loop {
        match rx.try_recv() {
            Ok(data) => break data,
            Err(mpsc::TryRecvError::Empty) => {
                if start.elapsed() > timeout_duration {
                    return Err(PipeError::EmptyPipe)
                }
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                return Err(PipeError::OtherErr("Reader thread died".into()));
            }
        }
    };
    
    if buffer.is_empty() {
        return Err(PipeError::EmptyPipe)
    }
    
    match String::from_utf8(buffer) {
        Ok(text) => Ok(text),
        Err(_) => Err(PipeError::BinaryPipe)
        
    }
}