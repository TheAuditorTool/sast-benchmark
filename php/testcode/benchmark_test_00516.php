<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00516(BenchmarkRequest $req): BenchmarkResponse {
    $headers = json_decode($req->bodyStr(), true)['headers'] ?? [];
    foreach ($headers as $h) {
        header($h);
    }
    return BenchmarkResponse::ok('headers set');
}
