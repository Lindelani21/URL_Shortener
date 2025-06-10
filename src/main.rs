use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use clap::{Parser, Subcommand};
use std::process;
use url_shortener::UrlShortener;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run as web server (default: CLI mode)
    #[arg(short, long)]
    server: bool,

    /// Port to run server on [default: 8000]
    #[arg(short, long, requires = "server")]
    port: Option<u16>,
}

#[derive(Subcommand)]
enum Commands {
    /// Shorten a URL (CLI mode)
    Shorten {
        /// The URL to shorten
        url: String,
    },
    /// Expand a short URL (CLI mode)
    Expand {
        /// The short code to expand
        code: String,
    },
    /// List all shortened URLs (CLI mode)
    List,
}

#[get("/{code}")]
async fn redirect(
    shortener: web::Data<UrlShortener>,
    code: web::Path<String>,
) -> impl Responder {
    match shortener.expand(&code) {
        Ok(url) => HttpResponse::PermanentRedirect()
            .append_header(("Location", url))
            .finish(),
        Err(_) => HttpResponse::NotFound().body("URL not found"),
    }
}

#[actix_web::main]
async fn run_server(shortener: UrlShortener, port: u16) -> std::io::Result<()> {
    println!("Server running on http://localhost:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shortener.clone()))
            .service(redirect)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let mut shortener = UrlShortener::new().unwrap_or_else(|e| {
        eprintln!("Failed to initialize: {}", e);
        process::exit(1);
    });

    if cli.server {
        // Web server mode
        let port = cli.port.unwrap_or(8000);
        run_server(shortener, port)
    } else {
        // CLI mode (original functionality)
        match cli.command {
            Some(Commands::Shorten { url }) => {
                match shortener.shorten(&url) {
                    Ok(code) => println!("Shortened: http://localhost:{}/{code}", cli.port.unwrap_or(8000)),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Some(Commands::Expand { code }) => {
                match shortener.expand(&code) {
                    Ok(url) => println!("Original URL: {}", url),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Some(Commands::List) => {
                let urls = shortener.list();
                if urls.is_empty() {
                    println!("No URLs stored");
                } else {
                    println!("Shortened URLs:");
                    for (code, url) in urls {
                        println!("http://localhost:{}/{code} -> {url}", cli.port.unwrap_or(8000));
                    }
                }
            }
            None => {
                eprintln!("No command provided. Use --help for usage.");
                process::exit(1);
            }
        }
        Ok(())
    }
}