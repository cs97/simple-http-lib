
use std::{
	fs,
	path::Path,
	io::{prelude::*, BufReader},
	net::TcpStream
};


pub struct RequestObj {
		request: String,
		path: String,
		pass: String,
		length: String,
	}



pub fn return_request_obj(mut stream: &TcpStream) -> RequestObj {

    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

	let request: &str = &extract_var("HTTP", 0, &http_request);
	let path: &str = &extract_var("HTTP", 1, &http_request)[1..];
	let pass: &str = &extract_var("Authorization", 2, &http_request);
	let length: &str = &extract_var("Content-Length", 1, &http_request);

	println!("Request from {} : {:#?}",stream.peer_addr().unwrap() , http_request);

	return RequestObj{request: request.to_string(), path: path.to_string(), pass: pass.to_string(), length: length.to_string()}
}



pub fn extract_var(s: &str, position: usize, http_request: &Vec<String>) -> String {
    for n in http_request{
        if n.contains(&s) {
            let content_length_vec: Vec<_> = n.split(' ').collect();
            return content_length_vec[position].to_string();
        }
    }
    return "none".to_string();
}

//-------------------------------------------------------------------------------------------

pub fn handle_get(mut path: &str, mut stream: TcpStream) {

    if path.len() < 1{
        path = "index.html";
    }

    if Path::new(path).is_file() {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read(path).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        stream.write_all(&contents).unwrap();

    } else {
        stream.write_all(not_found_404().as_bytes()).unwrap();
    }
}

fn handle_put(path: &str, content_length: &str, mut stream: TcpStream) {

    let length = content_length.parse::<usize>().unwrap();

    if path.len() < 1 {
        stream.write_all("HTTP/1.1 400 Bad Request\r\n".as_bytes()).unwrap();
        return
    }

    let mut buffer = vec![0; length];
    stream.read_exact(&mut buffer).unwrap();
    fs::write(path, buffer).unwrap();

    let status_line = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(status_line.as_bytes()).unwrap();
}

//-------------------------------------------------------------------------------------------

pub fn not_found_404() -> String {
//fn not_found_404(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 404 NOT FOUND";
    let contents = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
"#;

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    //stream.write_all(response.as_bytes()).unwrap();
    return response
}



pub fn unauthorized_401() -> String {
    let status_line = "HTTP/1.1 401 Unauthorized";
    let contents = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Unauthorized!</title>
  </head>
  <body>
    <h1>Unauthorized!</h1>
    <p>Sorry...</p>
  </body>
</html>
"#;

    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    //stream.write_all(response.as_bytes()).unwrap();
    return response
}



//-------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
	use std::net::TcpListener;

	#[test]
    fn it_works() {
    	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    	//let listener = TcpListener::bind("192.168.178.53:7878").unwrap();
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

}

