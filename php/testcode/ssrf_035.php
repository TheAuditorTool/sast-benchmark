<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_guzzle_base_uri
function ssrf035(BenchmarkRequest $req): BenchmarkResponse {
    $client = new GuzzleHttp\Client(['base_uri' => 'https://api.example.com']);
    $path = $req->param('path');
    $response = $client->get('/v1/' . $path); // vuln-code-snippet safe-line php_ssrf_guzzle_base_uri
    $body = (string)$response->getBody();
    return BenchmarkResponse::ok($body);
}
// vuln-code-snippet end php_ssrf_guzzle_base_uri
