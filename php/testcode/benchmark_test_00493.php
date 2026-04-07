<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00493(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $client = new \GuzzleHttp\Client();
    $response = $client->get($url);
    return BenchmarkResponse::ok((string) $response->getBody());
}
