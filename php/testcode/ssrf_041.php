<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_httpclient_allowlist
function ssrf041(BenchmarkRequest $req): BenchmarkResponse {
    $client = new HttpClient(['allowed_hosts' => ['api.example.com']]);
    $response = $client->get($req->param('path')); // vuln-code-snippet safe-line php_ssrf_httpclient_allowlist
    return BenchmarkResponse::ok($response->body());
}
// vuln-code-snippet end php_ssrf_httpclient_allowlist
