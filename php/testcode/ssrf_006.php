<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssrf_guzzle_config
function ssrf_guzzle_config(BenchmarkRequest $req): BenchmarkResponse {
    $endpoint = $req->param('endpoint');
    $client = new \GuzzleHttp\Client(['base_uri' => 'https://api.internal.com']);
    $response = $client->get('/v1/' . $endpoint); // vuln-code-snippet safe-line php_ssrf_guzzle_config
    return BenchmarkResponse::ok((string) $response->getBody());
}
// vuln-code-snippet end php_ssrf_guzzle_config
