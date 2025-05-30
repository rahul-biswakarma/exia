pub async fn submit_to_endpoint(
    endpoint: &str,
    data: &serde_json::Map<String, serde_json::Value>,
    config: &serde_json::Value,
) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        use gloo_net::http::Request;

        let method = config
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or("POST");
        let mut request = match method {
            "GET" => Request::get(endpoint),
            "POST" => Request::post(endpoint),
            "PUT" => Request::put(endpoint),
            "DELETE" => Request::delete(endpoint),
            _ => Request::post(endpoint),
        };

        if let Some(headers) = config.get("headers").and_then(|h| h.as_object()) {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request = request.header(key, value_str);
                }
            }
        }

        let response = request
            .json(data)
            .map_err(|e| format!("failed to serialize data: {}", e))?
            .send()
            .await
            .map_err(|e| format!("network error: {}", e))?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("http error: {}", response.status()))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let client = reqwest::Client::new();
        let method = config
            .get("method")
            .and_then(|m| m.as_str())
            .unwrap_or("POST");

        let mut request = match method {
            "GET" => client.get(endpoint),
            "POST" => client.post(endpoint),
            "PUT" => client.put(endpoint),
            "DELETE" => client.delete(endpoint),
            _ => client.post(endpoint),
        };

        if let Some(headers) = config.get("headers").and_then(|h| h.as_object()) {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request = request.header(key, value_str);
                }
            }
        }

        let response = request
            .json(data)
            .send()
            .await
            .map_err(|e| format!("network error: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!("http error: {}", response.status()))
        }
    }
}
