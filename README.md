# simple-http-lib


### example 

Cargo.toml
```
[dependencies]
simple-http-lib = { git = "https://github.com/cs97/simple-http-lib" }
```
main.rs
```
use std::net::TcpListener;
use std::io::Write;
use simple_http_lib::*;


fn main() {

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	for stream in listener.incoming() {
		let mut stream = stream.unwrap();

		let request_obj: RequestObj = return_request_obj(&stream);

		// Example authentication. Please use a secure authentication method
		/*
		let pass = "none";
		if request_obj.pass.to_string() != pass {
			stream.write_all(unauthorized_401().as_bytes()).unwrap();
			return
		}
		*/

		match request_obj.request.as_str() {
			"GET" => handle_get(&request_obj.path, stream),
			"PUT" => handle_put(&request_obj.path, &request_obj.length, stream),
			_ => stream.write_all(not_found_404().as_bytes()).unwrap(),
		}

	}
}
```
