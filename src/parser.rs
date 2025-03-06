use std::collections::HashMap;

pub fn parse_request(request: &str) -> (HashMap<String, String>, String, HashMap<String, String>) {
    let mut headers = HashMap::new();
    let mut body = String::new();
    let mut query_params = HashMap::new();

    let mut lines = request.lines();

    if let Some(request_line) = lines.next() {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() > 1 {
            if let Some((_, query)) = parts[1].split_once('?') {
                query_params = query
                    .split('&')
                    .filter_map(|pair| pair.split_once('='))
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect();
            }
        }
    }

    for line in lines.clone() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    body = lines.collect::<Vec<&str>>().join("\r\n");

    (headers, body, query_params)
}
