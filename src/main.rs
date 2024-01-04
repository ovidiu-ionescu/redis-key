use redis::Commands;
use std::io::{self, Write};
use std::env;
use zune_inflate::DeflateDecoder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <url> <key> [<file>]", args[0]);
        return Err("Invalid arguments".into());
    }

    let url = args[1].clone();
    let key = &args[2];

    let client = redis::Client::open(url)?;
    let mut con = client.get_connection()?;
    if args.len() == 4 {
        let filename = &args[3];
        let value = std::fs::read(filename)?;
        write_key(&mut con, key, &value)?;
    } else {
        read_key_to_stdout(&mut con, key)?;
    }
    Ok(())
}

fn read_key_to_stdout(con: &mut redis::Connection, key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let x: Vec<u8> = read_key(con, key)?;
    io::stdout().write_all(&x).unwrap();
    Ok(())
}

fn read_key(con: &mut redis::Connection, key: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let x: Vec<u8> = con.get(key)?;
    if x.is_empty() {
        return Err("Key not found".into());
    }
    //check if it is gzipped
    if x[0] == 0x1f && x[1] == 0x8b {
        let mut decoder = DeflateDecoder::new(&x[..]);
        let decompressed_data = decoder.decode_gzip()?;
        Ok(decompressed_data)
    } else {
        Ok(x)
    }
}

fn write_key(con: &mut redis::Connection, key: &str, value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    con.set(key, value)?;
    Ok(())
}

