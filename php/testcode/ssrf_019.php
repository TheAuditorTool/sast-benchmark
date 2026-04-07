<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_guzzle_request_method
function ssrf019(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $client = new GuzzleHttp\Client();
    $response = $client->request('GET', $url); // vuln-code-snippet vuln-line php_ssrf_guzzle_request_method
    $body = (string)$response->getBody();
    return BenchmarkResponse::ok($body);
}
// vuln-code-snippet end php_ssrf_guzzle_request_method
