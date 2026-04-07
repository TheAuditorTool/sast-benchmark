<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00779(BenchmarkRequest $req): BenchmarkResponse {
    $client = new GuzzleHttp\Client(['base_uri' => 'https://api.example.com']);
    $path = $req->param('path');
    $response = $client->get('/v1/' . $path);
    $body = (string)$response->getBody();
    return BenchmarkResponse::ok($body);
}
