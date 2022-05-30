use prost::Message;
use protoc_gen_prost::GeneratorResultExt;
use std::{
    env,
    io::{self, Read, Write},
    process::exit,
};

fn main() -> io::Result<()> {
    if let Some(_) = env::args().find(|x| x == "--version") {
        println!(env!("CARGO_PKG_VERSION"));
        exit(0);
    }

    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf)?;

    let response = protoc_gen_tonic::execute(buf.as_slice()).unwrap_codegen_response();

    buf.clear();
    response.encode(&mut buf).expect("error encoding response");
    io::stdout().write_all(&buf)?;

    Ok(())
}
