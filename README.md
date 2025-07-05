# reqse
HTTP Request and Response

## Motivation
Making it easy to parse a `&[u8]` into a HTTP Request / Response for further
handling. Or converting a HTTP Request / Responses into a `&[u8]` to send them
via TCP. This makes it very easy to build a KISS HTTP/1.1 web server without
using heavy frameworks like axum, actix or rocket. 

A very stupid implementation of such a server can be seen here
(examples/hello-world.rs)
```rust
fn main() {
    let listener = TcpListener::bind("localhost:3000").unwrap();
    let mut buf = [0 as u8; 1028];

    loop {
        let (mut connection, _) = listener.accept().unwrap();

        // read the request
        connection.read(&mut buf).unwrap();
        
        // parse the request
        let request = Request::from_bytes(&buf).unwrap();

        // create a response based on the request in this case either
        // - 200 Ok with body: "Hello World"
        // - or 404 NotFound
        let response = match (request.method, request.uri.as_str()) {
            (Method::Get, "/") => Response::ok()
                .body("Hello World".as_bytes().to_vec())
                .finish(),
            _ => Response::not_found().finish(),
        };

        // send the response to the client
        connection.write(&response.to_bytes()).unwrap();
    }
}
```
