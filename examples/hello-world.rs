use reqse::{Method, Request, ResponseBuilder};
use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // create a blocking TcpListener
    let listener = TcpListener::bind("localhost:3000").unwrap();

    // create a buffer for reading the requests later
    let mut buf = [0 as u8; 1028];

    loop {
        // wait until a client connects via tcp
        //
        // NOTE: this example is not async if a client connects other clients have to wait until
        // first request is processed
        let (mut connection, _) = listener.accept().unwrap();

        println!("accepted connection");

        // read request from the connection
        //
        // NOTE: this read is not guranteed to read the entire request in a real application a
        // reading loop would be more appropriate see example/hello-world-loop.rs
        connection.read(&mut buf).unwrap();

        // create a reqse::Request from the read bytes
        let request = Request::from_bytes(&buf).unwrap();

        println!("got request: {:#?}", &request);

        // check if the request is  a GET request on '/' if so return 200 OK with body 'Hello
        // World' otherwise return 404 Not Found
        let response = match (request.method(), request.uri()) {
            (Method::Get, "/") => {
                let mut response = ResponseBuilder::ok();
                response.body_mut().extend_from_slice(b"Hello World");
                response
            }
            _ => ResponseBuilder::not_found(),
        };

        println!("created response: {:#?}", &response);

        // send response to client
        //
        // NOTE: the write method might not write the entire response in a real application a write
        // loop would be more appropriate
        connection.write(response.finish().as_ref()).unwrap();

        println!("send response");
    }
}
