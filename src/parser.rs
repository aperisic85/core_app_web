use std::collections::HashMap;

pub fn parse_request(request: &str) -> (HashMap<String, String>, String) {
    let mut headers = HashMap::new();
    let mut body = String::new();

    let mut lines = request.split("\r\n");

    if let Some(_) = lines.next() {} // Skip request line (e.g., GET / HTTP/1.1)

    for line in &mut lines {
        if line.is_empty() {
            break; // Empty line means headers are done
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    body = lines.collect::<Vec<&str>>().join("\r\n");
    (headers, body)
}
