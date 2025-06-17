use actix_web::{get, web, App, HttpResponse, HttpServer};
use clap::{Parser, Subcommand};
use std::sync::Arc;
use url_shortener::UrlShortener;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Run as web server
    #[arg(short, long)]
    server: bool,

    /// Server port [default: 8000]
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

async fn run_web_server(shortener: Arc<UrlShortener>, port: u16) -> std::io::Result<()> {
    println!("🌐 Server running at: http://localhost:{port}");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shortener.clone()))
            .service(redirect)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let shortener = Arc::new(UrlShortener::new().expect("Failed to initialize"));

    if cli.server {
        run_web_server(shortener, cli.port).await
    } else {
        match cli.command {
            Some(Commands::Shorten { url }) => {
                match shortener.shorten(&url) {
                    Ok(code) => println!("🔗 http://localhost:{}/{code}", cli.port),
                    Err(e) => eprintln!("❌ {e}"),
                }
            }
            Some(Commands::Expand { code }) => {
                match shortener.expand(&code) {
                    Ok(url) => println!("🔗 {url}"),
                    Err(e) => eprintln!("❌ {e}"),
                }
            }
            Some(Commands::List) => {
                for (code, url) in shortener.list() {
                    println!("➡️  http://localhost:{}/{code} -> {url}", cli.port);
                }
            }
            None => eprintln!("ℹ️  Use --help for usage"),
        }
        Ok(())
    }
}
