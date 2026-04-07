<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00525(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $client = new GuzzleHttp\Client();
    $response = $client->request('GET', $url);
    $body = (string)$response->getBody();
    return BenchmarkResponse::ok($body);
}
