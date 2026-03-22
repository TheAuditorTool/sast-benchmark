//! Network operations demonstrating SSRF sinks.
//!
//! All network operations with user-controlled URLs/addresses are SSRF vectors.

use crate::models::ProxyRequest;
use std::io;
use std::net::TcpStream;

/// TAINT SINK: reqwest::get with user-controlled URL (SSRF)
// vuln-code-snippet start ssrfFetchUrl
pub async fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
    // TAINT SINK: User-controlled URL (SSRF vulnerability!)
    let response = reqwest::get(url).await?; // vuln-code-snippet target-line ssrfFetchUrl
    response.text().await
}
// vuln-code-snippet end ssrfFetchUrl

// vuln-code-snippet start ssrfFetchUrl2
///Domain allowlist check before fetching
pub async fn fetch_url_safe(url: &str, allowed_domains: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = url::Url::parse(url)?;
    let host = parsed.host_str().ok_or("No host in URL")?;
    if !allowed_domains.iter().any(|d| host == *d) { // vuln-code-snippet target-line ssrfFetchUrl2
        return Err(format!("Domain '{}' not in allowlist", host).into());
    }
    let response = reqwest::get(url).await?;
    Ok(response.text().await?)
}
// vuln-code-snippet end ssrfFetchUrl2

/// TAINT SINK: reqwest::Client::get with user-controlled URL (SSRF)
pub async fn fetch_url_with_client(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    // TAINT SINK: User-controlled URL via Client::get
    let response = client.get(url).send().await?;
    response.text().await
}

/// TAINT SINK: Full proxy request with user-controlled options
pub async fn fetch_url_with_options(request: &ProxyRequest) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    // TAINT SINK: User-controlled URL (SSRF!)
    let mut req = client.get(&request.url);

    // Add user-controlled headers (header injection risk)
    if let Some(ref headers) = request.headers {
        for (key, value) in headers {
            req = req.header(key.as_str(), value.as_str());
        }
    }

    let response = req.send().await?;
    response.text().await
}

/// TAINT SINK: POST request with user-controlled URL and body
// vuln-code-snippet start ssrfPostToUrl
pub async fn post_to_url(url: &str, body: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    // TAINT SINK: User-controlled URL and body
    let response = client
        .post(url) // vuln-code-snippet target-line ssrfPostToUrl
        .body(body.to_string())
        .send()
        .await?;

    response.text().await
}
// vuln-code-snippet end ssrfPostToUrl

// vuln-code-snippet start ssrfPostToUrl2
///Domain allowlist + body size limit before posting
pub async fn post_to_url_safe(url: &str, body: &str, allowed_domains: &[&str], max_body_size: usize) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = url::Url::parse(url)?;
    let host = parsed.host_str().ok_or("No host in URL")?;
    if !allowed_domains.iter().any(|d| host == *d) { // vuln-code-snippet target-line ssrfPostToUrl2
        return Err(format!("Domain '{}' not in allowlist", host).into());
    }
    if body.len() > max_body_size {
        return Err(format!("Body size {} exceeds limit {}", body.len(), max_body_size).into());
    }
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .body(body.to_string())
        .send()
        .await?;
    Ok(response.text().await?)
}
// vuln-code-snippet end ssrfPostToUrl2

/// TAINT SINK: TcpStream::connect with user-controlled address (SSRF)
// vuln-code-snippet start ssrfConnectTcp
pub fn connect_tcp(host: &str, port: u16) -> io::Result<TcpStream> {
    let address = format!("{}:{}", host, port);

    // TAINT SINK: User-controlled network connection
    TcpStream::connect(address) // vuln-code-snippet target-line ssrfConnectTcp
}
// vuln-code-snippet end ssrfConnectTcp

// vuln-code-snippet start ssrfConnectTcp2
///Allowlisted host:port pairs + reject private IPs
pub fn connect_tcp_safe(host: &str, port: u16, allowed_endpoints: &[(&str, u16)]) -> io::Result<TcpStream> {
    if !allowed_endpoints.iter().any(|(h, p)| *h == host && *p == port) { // vuln-code-snippet target-line ssrfConnectTcp2
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, format!("Endpoint {}:{} not in allowlist", host, port)));
    }
    if let Ok(ip) = host.parse::<std::net::IpAddr>() {
        let is_private = match ip {
            std::net::IpAddr::V4(v4) => v4.is_loopback() || v4.is_private() || v4.is_link_local(),
            std::net::IpAddr::V6(v6) => v6.is_loopback(),
        };
        if is_private {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Private IP addresses are blocked"));
        }
    }
    let address = format!("{}:{}", host, port);
    TcpStream::connect(address)
}
// vuln-code-snippet end ssrfConnectTcp2

/// TAINT SINK: Direct socket connection
pub fn connect_to_address(address: &str) -> io::Result<TcpStream> {
    // TAINT SINK: std::net::TcpStream::connect with user address
    std::net::TcpStream::connect(address)
}

/// Parse URL and extract host (still dangerous if URL is tainted)
pub fn extract_host_from_url(url: &str) -> Option<String> {
    // Parsing tainted URL
    url.strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))
        .and_then(|rest| rest.split('/').next())
        .map(String::from)
}

/// TAINT SINK: DNS lookup with user-controlled hostname
pub fn resolve_hostname(hostname: &str) -> io::Result<Vec<std::net::IpAddr>> {
    use std::net::ToSocketAddrs;

    // TAINT SINK: User-controlled hostname resolution
    let address = format!("{}:80", hostname);
    let addrs: Vec<_> = address
        .to_socket_addrs()?
        .map(|addr| addr.ip())
        .collect();

    Ok(addrs)
}

/// Webhook sender (common SSRF vector in applications)
// vuln-code-snippet start ssrfSendWebhook
pub async fn send_webhook(webhook_url: &str, payload: &str) -> Result<u16, reqwest::Error> {
    let client = reqwest::Client::new();

    // TAINT SINK: User-configured webhook URL (SSRF!)
    let response = client
        .post(webhook_url) // vuln-code-snippet target-line ssrfSendWebhook
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?;

    Ok(response.status().as_u16())
}
// vuln-code-snippet end ssrfSendWebhook

// vuln-code-snippet start ssrfSendWebhook2
///Webhook URL checked against trusted webhook list
pub async fn send_webhook_safe(webhook_url: &str, payload: &str, trusted_webhooks: &[&str]) -> Result<u16, Box<dyn std::error::Error>> {
    if !trusted_webhooks.iter().any(|w| webhook_url == *w) { // vuln-code-snippet target-line ssrfSendWebhook2
        return Err(format!("Webhook URL '{}' not in trusted list", webhook_url).into());
    }
    let client = reqwest::Client::new();
    let response = client
        .post(webhook_url)
        .header("Content-Type", "application/json")
        .body(payload.to_string())
        .send()
        .await?;
    Ok(response.status().as_u16())
}
// vuln-code-snippet end ssrfSendWebhook2

/// URL redirect follower (can lead to internal network access)
pub async fn follow_redirect(url: &str, max_redirects: u8) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(max_redirects as usize))
        .build()?;

    // TAINT SINK: Following redirects from user-provided URL
    // Could redirect to internal services!
    let response = client.get(url).send().await?;
    response.text().await
}

/// Image/Resource fetcher (common in content platforms)
pub async fn fetch_image(image_url: &str) -> Result<Vec<u8>, reqwest::Error> {
    // TAINT SINK: Fetching from user-provided URL
    // Could access internal resources, cloud metadata, etc.
    let response = reqwest::get(image_url).await?;
    let bytes = response.bytes().await?;
    Ok(bytes.to_vec())
}

/// URL validator (insufficient - can still be bypassed)
pub fn validate_url(url: &str) -> bool {
    // WARNING: This validation is INSUFFICIENT!
    // Many bypasses exist (DNS rebinding, URL parsing inconsistencies, etc.)

    // Basic checks that can be bypassed
    if url.starts_with("http://") || url.starts_with("https://") {
        // Check for obvious internal addresses
        let lower = url.to_lowercase();
        if lower.contains("localhost")
            || lower.contains("127.0.0.1")
            || lower.contains("0.0.0.0")
            || lower.contains("169.254.")
            || lower.contains("10.")
            || lower.contains("192.168.")
            || lower.contains("[::1]")
        {
            return false;
        }
        return true;
    }
    false
}

/// SSRF via URL with basic validation (still vulnerable!)
// vuln-code-snippet start ssrfFetchExternalOnly
pub async fn fetch_external_only(url: &str) -> Result<String, String> {
    if !validate_url(url) {
        return Err("Invalid or internal URL".to_string());
    }

    // Still vulnerable to:
    // - DNS rebinding attacks
    // - URL parsing differences
    // - IPv6 addresses
    // - Decimal IP addresses (e.g., http://2130706433)
    // - URL redirects to internal resources
    fetch_url(url).await.map_err(|e| e.to_string()) // vuln-code-snippet target-line ssrfFetchExternalOnly
}
// vuln-code-snippet end ssrfFetchExternalOnly

// vuln-code-snippet start ssrfFetchExternalOnly2
///Strict URL parse, reject internal, no redirects, reject decimal IPs
pub async fn fetch_external_only_safe(url: &str, allowed_domains: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let parsed = url::Url::parse(url)?;
    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return Err("Only http/https schemes allowed".into());
    }
    let host = parsed.host_str().ok_or("No host in URL")?;
    if host.chars().all(|c| c.is_ascii_digit() || c == '.') {
        return Err("Decimal/numeric IP addresses are rejected".into());
    }
    if !allowed_domains.iter().any(|d| host == *d) { // vuln-code-snippet target-line ssrfFetchExternalOnly2
        return Err(format!("Domain '{}' not in allowlist", host).into());
    }
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let response = client.get(url).send().await?;
    Ok(response.text().await?)
}
// vuln-code-snippet end ssrfFetchExternalOnly2
