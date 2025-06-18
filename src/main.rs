use actix_web::{get, web, App, HttpResponse, HttpServer};
use clap::{Parser, Subcommand};
use std::sync::Arc;
use url_shortener::UrlShortener;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    server: bool,

    #[arg(short, long, default_value_t = 8000)]
    port: u16,
}

#[derive(Subcommand)]
enum Commands {
    Shorten { url: String },
    Expand { code: String },
    List,
}

#[get("/{code}")]
async fn redirect(
    data: web::Data<Arc<UrlShortener>>,
    code: web::Path<String>,
) -> HttpResponse {
    match data.expand(&code) {
        Ok(url) => HttpResponse::MovedPermanently()
            .append_header(("Location", url))
            .finish(),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.server {
        let shortener = Arc::new(UrlShortener::new()?);
        println!("Server running at: http://localhost:{}", cli.port);
        println!("Data file: {}", std::env::current_dir()?.join("data.json").display());
        
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(shortener.clone()))
                .service(redirect)
        })
        .bind(("0.0.0.0", cli.port))?
        .run()
        .await?;
    } else {
        let shortener = UrlShortener::new()?;
        match cli.command {
            Some(Commands::Shorten { url }) => {
                let code = shortener.shorten(&url)?;
                println!(" Shortened URL created:");
                println!(" Short code: {}", code);
                println!(" Full URL: http://localhost:{}/{}", cli.port, code);
                println!(" Stored in: {}", std::env::current_dir()?.join("data.json").display());
            }
            Some(Commands::Expand { code }) => {
                let url = shortener.expand(&code)?;
                println!("Original URL: {}", url);
            }
            Some(Commands::List) => {
                println!("All shortened URLs:");
                for (code, url) in shortener.list() {
                    println!("  {} -> {}", code, url);
                }
            }
            None => {
                eprintln!("No command provided");
                eprintln!("Usage:");
                eprintln!("  cargo run -- shorten <URL>");
                eprintln!("  cargo run -- expand <CODE>");
                eprintln!("  cargo run -- list");
                eprintln!("  cargo run -- --server");
            }
        }
    }
    Ok(())
}
