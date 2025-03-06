use std::collections::HashMap;
use crate::error::AppError;

pub fn parse_request(request: &str) -> Result<(HashMap<String, String>, String, HashMap<String, String>), AppError> {
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
        } else {
            return Err(AppError::ParseError("Invalid request line".to_string()));
        }
    }

    for line in lines.clone() {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        } else {
            return Err(AppError::ParseError(format!("Invalid header format: {}", line)));
        }
    }

    body = lines.collect::<Vec<&str>>().join("\r\n");

    Ok((headers, body, query_params))
}
