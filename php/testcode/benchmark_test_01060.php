<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01060(BenchmarkRequest $req): BenchmarkResponse {
    $endpoint = $req->param('endpoint');
    $client = new \GuzzleHttp\Client(['base_uri' => 'https://api.internal.com']);
    $response = $client->get('/v1/' . $endpoint);
    return BenchmarkResponse::ok((string) $response->getBody());
}
