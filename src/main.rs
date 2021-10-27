use rustkernel::handlers;
use rustkernel::Program;
use std::net::TcpListener;

fn main() {
    // Start the server
    let host: &str = "127.0.0.1:8787";
    let listener = TcpListener::bind(host).expect("Could not start listener");
    println!("Rustkernel is running on {}", host);

    // The Program will remain in state while the server is running
    let mut program = Program::new();

    println!("Ctrl+Click to view: {}/main.rs", &program.temp_dir);
    println!("Ctrl+Click to view: {}/Cargo.toml", &program.temp_dir);

    // This is the main loop over incoming streams
    for stream in listener.incoming() {
        let stream = stream.expect("Could not iterate over stream");
        handlers::code_request(stream, &mut program);
    }
}
