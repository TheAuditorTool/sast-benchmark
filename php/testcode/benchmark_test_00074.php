<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00074(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $allowedDomains = ['api.example.com', 'cdn.example.com'];
    $host = parse_url($url, PHP_URL_HOST);
    if (!in_array($host, $allowedDomains, true)) {
        return BenchmarkResponse::badRequest("Domain not allowed");
    }
    $ch = curl_init();
    curl_setopt($ch, CURLOPT_URL, $url);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    curl_setopt($ch, CURLOPT_FOLLOWLOCATION, false);
    $response = curl_exec($ch);
    curl_close($ch);
    return BenchmarkResponse::ok($response);
}
