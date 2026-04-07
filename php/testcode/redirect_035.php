<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_parse_url_allowlist
function redirect035(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $p = parse_url($url);
    $allowed = ['example.com', 'app.example.com'];
    if (!in_array($p['scheme'] ?? '', ['http', 'https'], true) || !in_array($p['host'] ?? '', $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    header('Location: ' . $url); // vuln-code-snippet safe-line php_redirect_parse_url_allowlist
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_parse_url_allowlist
