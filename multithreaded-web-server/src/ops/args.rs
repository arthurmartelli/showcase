use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct ServerOptions {
    /// Sets the port for the web server to use
    #[clap(short, long, default_value_t = 8080)]
    pub port: u32,

    /// Sets the directory for the html pages
    #[clap(short, long, default_value_t = String::from("public"))]
    pub dir: String,
}
