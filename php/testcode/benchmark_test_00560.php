<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00560(BenchmarkRequest $req): BenchmarkResponse {
    $client = new HttpClient(['allowed_hosts' => ['api.example.com']]);
    $response = $client->get($req->param('path'));
    return BenchmarkResponse::ok($response->body());
}
