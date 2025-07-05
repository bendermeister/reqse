use reqse::{Method, Request, Response};
use std::io;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("localhost:3000").await?;

    loop {
        let (connection, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = handle_connection(connection).await {
                eprintln!("ERROR in connection: {}", err);
            }
        });
    }
}

async fn handle_connection(mut connection: TcpStream) -> io::Result<()> {
    let mut buf = [0 as u8; 1028];

    println!("start handling connection");

    'conn: loop {
        let mut buf_offset = 0;

        let request = 'read: loop {
            println!("reading request");
            if buf_offset >= buf.len() {
                connection
                    .write_all(&Response::bad_request().finish().to_bytes())
                    .await?;
                continue 'conn;
            }

            buf_offset += match connection.read(&mut buf[buf_offset..]).await? {
                0 => break 'conn,
                n => n,
            };

            match Request::from_bytes(&buf) {
                Ok(req) => break 'read req,
                Err(reqse::Error::NotEnoughData) => (),
                Err(err) => {
                    eprintln!("ERROR while parsing request: {}", err);
                    connection
                        .write_all(&Response::bad_request().finish().to_bytes())
                        .await?;
                    continue 'conn;
                }
            }
        };

        println!("got request: {:#?}", &request);

        let response =
            router(request).unwrap_or_else(|_| Response::internal_server_error().finish());

        println!("created response: {:#?}", &response);
        connection.write_all(response.to_bytes().as_ref()).await?;
        connection.flush().await?;
    }

    println!("client closed connection");

    Ok(())
}

fn router(request: Request) -> io::Result<Response> {
    match (request.method, request.uri.as_ref()) {
        (Method::Get, "/") => routes::root(request),

        (Method::Get, "/health_check") => routes::health_check(request),

        _ => Ok(Response::not_found().finish()),
    }
}

mod routes {
    use super::*;
    pub fn root(_: Request) -> io::Result<Response> {
        Ok(Response::ok()
            .body("Hello World".as_bytes().to_vec())
            .finish())
    }

    pub fn health_check(_: Request) -> io::Result<Response> {
        Ok(Response::ok().finish())
    }
}
