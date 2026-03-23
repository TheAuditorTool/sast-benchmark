<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_guzzle_user_url
function ssrf_guzzle_user_url(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $client = new \GuzzleHttp\Client();
    $response = $client->get($url); // vuln-code-snippet vuln-line php_ssrf_guzzle_user_url
    return BenchmarkResponse::ok((string) $response->getBody());
}
// vuln-code-snippet end php_ssrf_guzzle_user_url
