use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

mod error;
mod ghost;

use error::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init_custom_env("GHOPHER_LOG_LEVEL");

    let result = tokio::select! {
        result = run_server() => result,
        _ = tokio::signal::ctrl_c() => Ok(())
    };

    if let Err(err) = result {
        log::error!("{}", err);
        std::process::exit(1);
    }
}

async fn run_server() -> Result<()> {
    let listener = create_listener().await?;

    loop {
        let (conn, addr) = listener.accept().await?;
        log::info!("connection received from `{}`", addr);

        tokio::spawn(async move {
            if let Err(err) = handle_client(conn).await {
                log::error!("{}", err);
            }
        });
    }
}

async fn create_listener() -> Result<TcpListener> {
    let addr = dotenv::var("GHOPHER_BIND_ADDR").unwrap_or(String::from("0.0.0.0"));
    let port = dotenv::var("GHOPHER_BIND_PORT").unwrap_or(String::from("70"));
    let binding = format!("{}:{}", addr, port);

    let listener = TcpListener::bind(&binding).await?;

    log::info!("listening on `{}`", binding);

    Ok(listener)
}

async fn handle_client(mut conn: TcpStream) -> Result<()> {
    let (reader, mut writer) = conn.split();
    let mut reader = BufReader::new(reader);

    if let Some(path) = read_request(&mut reader).await? {
        let response = handle_request(path).await?;
        writer.write_all(response.as_bytes()).await?;
    }

    Ok(())
}

async fn read_request<R: AsyncBufReadExt + Unpin>(reader: &mut R) -> Result<Option<String>> {
    let mut line = String::new();
    if reader.read_line(&mut line).await? == 0 {
        return Ok(None);
    }

    let request_parts = line.splitn(2, '\t').collect::<Vec<&str>>();

    Ok(Some(String::from(request_parts[0].trim())))
}

async fn handle_request(path: String) -> Result<String> {
    log::info!("Request: `{}`", path);

    if path.is_empty() {
        menu().await
    } else {
        page(path).await
    }
}

async fn menu() -> Result<String> {
    let settings = ghost::get_settings().await?;
    
    let mut response = String::new();
    response.push_str(&format!("i{}\tfake\t(NULL)\t0\r\n", settings.title));
    response.push_str("i\tfake\t(NULL)\t0\r\n");
    response.push_str(&format!("i{}\tfake\t(NULL)\t0\r\n", settings.description));
    response.push_str("i\tfake\t(NULL)\t0\r\n");
    response.push_str("i\tfake\t(NULL)\t0\r\n");

    let pages = ghost::get_pages().await?;
    for page in pages {
        response.push_str(&format!("0{}\t{}\t{}\t{}\r\n",
            page.title,
            page.slug,
            dotenv::var("GHOPHER_HOST")?,
            dotenv::var("GHOPHER_BIND_PORT").unwrap_or(String::from("70"))));
    }

    response.push_str("i\tfake\t(NULL)\t0\r\n");

    let posts = ghost::get_posts().await?;
    for post in posts {
        response.push_str(&format!("0[{}] {}\t{}\t{}\t{}\r\n",
            post.published_at.date(),
            post.title,
            post.slug,
            dotenv::var("GHOPHER_HOST")?,
            dotenv::var("GHOPHER_BIND_PORT").unwrap_or(String::from("70"))));
    }

    response.push_str(".\r\n");

    Ok(response)
}

async fn page(path: String) -> Result<String> {
    let content = ghost::get_content(&path).await?;

    let mut response = String::new();

    response.push_str(&content.title);
    response.push_str("\r\n\r\n");
    response.push_str(&august::convert(&content.html, 70));
    response.push_str("\r\n");

    Ok(response)
}
