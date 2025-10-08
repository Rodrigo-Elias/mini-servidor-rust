//! # An example of an static HTTP mini-server in RUST
//!
//! Implements a simple, asynchronous HTTP server in Rust,
//! using the Tokio library. The server listens on port 8080 and
//! is capable of serving static files from the project's `public/` folder.

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;
use mime_guess;
use std::path::{PathBuf};

/// Main server entry point.
///
/// Configures the TCP server on port 8080 and enters in an infinite loop,
/// accepting connections and delegating processing to asynchronous tasks.
///
/// # Errors
/// Returns an error if the server cannot bind to port 8080
/// or if there is an error accepting the connection.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Servidor rodando em http://127.0.0.1:8080");

    loop {
        // loop not blocked because of the await
        let (stream, _) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Falha ao processar conexão: {}", e);
            }
        });
    }
}

/// Handles a single TCP connection and parses the HTTP request.
///
/// Reads the first line of the request to extract the method (must be GET)
/// and the path of the requested file, delegating the response to the `serve_file` function.
///
/// # Arguments
/// * `stream` - The received TCP connection, passed as mutable for reading and writing.
///
/// # Return
/// Returns `Ok(())` after processing or discarding the request.
/// 
async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024]; //Eh, o suficiente.
    stream.read(&mut buffer).await?;

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    // Se a requisição não tiver o formato esperado (GET /caminho), a conexão é encerrada.
    if parts.len() < 2 || parts[0] != "GET" {
        return Ok(());
    }

    let path = parts[1];
    serve_file(&mut stream, path).await;
    Ok(())
}

/// Serves a static file for the TCP connection.
///
/// Resolves the file path in the `public` folder. If the `path` is `“/”` (root),
/// serves `index.html`. If the file is not found, sends an HTTP 404 response.
///
/// # Arguments
/// * `stream` - A mutable reference to the TCP connection, used to send the response.
/// * `path` - The path of the resource requested in the HTTP request (e.g., “/style.css”).
///
/// # Panics
/// Panics if there is a failure writing the HTTP response
/// or if reading the file is successful but the stream fails.
async fn serve_file(stream: &mut TcpStream, path: &str) {
    let mut file_path = PathBuf::from("public");

    // Lógica de roteamento: Se for raiz, aponta para index.html
    if path == "/" {
        file_path.push("index.html");
    } else {
        // Remove a barra inicial e constrói o caminho para outros arquivos
        file_path.push(path.trim_start_matches('/'));
    }

    let file_content = fs::read(&file_path).await;
    
    if let Ok(content) = file_content {
        // Sucesso (HTTP 200 OK)
        let mime_type = mime_guess::from_path(&file_path).first_or_text_plain();
        let status_line = "HTTP/1.1 200 OK\r\n";
        let headers = format!("Content-Type: {}\r\nContent-Length: {}\r\n\r\n", mime_type, content.len());

        stream.write_all(status_line.as_bytes()).await.unwrap();
        stream.write_all(headers.as_bytes()).await.unwrap();
        stream.write_all(&content).await.unwrap();
    } else {
        // Falha (HTTP 404 NOT FOUND)
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let not_found_page = "<h1>404 - Not Found uwehehe</h1>";
        stream.write_all(status_line.as_bytes()).await.unwrap();
        stream.write_all(not_found_page.as_bytes()).await.unwrap();
    }
}