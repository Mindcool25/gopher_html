use std::net::TcpStream;
use std::io::Write;
use std::io::Read;

cgi::cgi_main! { |request: cgi::Request| -> cgi::Response {
    let entry = request.headers().get("x-cgi-query-string").unwrap().to_str().expect("Failed");
    let url: Vec<&str> = entry.split('*').collect();
    let page: String;
    
    if url[0].is_empty() {
        page = get_gopher_page("192.168.1.154:70".to_string(), "".to_string());
    }
    else if url[0].starts_with('1'){
        page = get_gopher_page(url[2].to_string(), url[1].to_string());
    }
    else {
        page = get_text_file(url[2].to_string(), url[1].to_string());
    }
    
    cgi::html_response(200, page)
}
}

fn get_gopher_page(ip: String, request: String) -> String {
    // Connecting to server and sending request
    let mut stream = TcpStream::connect(ip).expect("Failed to get page.");
    stream.write_all(format!("\r{}", request).as_bytes()).expect("Failed to send.");

    // Reading from server
    let mut buf = [0;32768];
    let _bytes_read = stream.read(&mut buf);
    let in_string = String::from_utf8_lossy(&buf).replace(&['\r', '\u{0}'][..], "");

    // Parsing the inputs
    let mut gm: Vec<&str> = in_string.as_str().split('\n').collect();
    let mut out: String = "<head><link rel = \"stylesheet\" href=\"../main.css\"></head>".to_string();
    for i in gm.iter_mut() {
        if i.starts_with('i') {
            let line = &i[1..i.len()];
            out += &format!("<p>{}</p>\n", line).to_string();
        }
        else if i.starts_with('0') {
            let line = &i[1..i.len()];
            let split_line: Vec<&str> = line.split('\t').collect();
            out += &format!("<p><a href=?0*{}*{}:{}>TXT</a> \t{}</p>",
                split_line[1],
                split_line[2],
                split_line[3],
                split_line[0]);
        }
        else if i.starts_with('1') {
            let line = &i[1..i.len()];
            let split_line: Vec<&str> = line.split('\t').collect();
            out += &format!("<p><a href=?1*{}*{}:{}>MAP</a> \t{}</p>",
                split_line[1],
                split_line[2],
                split_line[3],
                split_line[0]);
        }
    }
    out
}

fn get_text_file(ip: String, request: String) -> String {
    // Connecting to server and sending request
    let mut stream = TcpStream::connect(ip).expect("Failed to get page.");
    stream.write_all(format!("\r{}", request).as_bytes()).expect("Failed to send.");

    // Reading from server
    let mut buf = [0;32768];
    let _bytes_read = stream.read(&mut buf);
    let mut out = "<head><link rel = \"stylesheet\" href=\"../main.css\"></head>".to_string();
    out += &format!("<p>{}</p>", String::from_utf8_lossy(&buf).replace(&['\r', '\u{0}'][..], ""));
    out.replace('\n', "<br>")
}
