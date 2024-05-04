use std::net::TcpListener;

use web_server::ThreadPool;

use self::connection::Server;

pub mod args;
pub mod connection;

/// Handles the arguments passed into the server,
/// uses the default values accordingly
pub fn command_handler(args: args::ServerOptions) {
    let server = connection::Server::new(args.dir, args.port);
    run(server)
}

/// Creates and runs the server
fn run(server: Server) {
    let addr = format!("127.0.0.1:{}", &server.port());
    let responses = server.responses();

    let listener = TcpListener::bind(addr.as_str()).unwrap();
    let pool = ThreadPool::new(4);

    println!("go to http://{addr}");

    for stream in listener.incoming() {
        // use .take(i) to limit to i requests
        // or create another way to break the loop
        let stream = stream.unwrap();
        let res = responses.clone();

        pool.execute(move || {
            connection::handle_connection(&res, stream);
        })
    }

    println!("Shutting down.")
}
