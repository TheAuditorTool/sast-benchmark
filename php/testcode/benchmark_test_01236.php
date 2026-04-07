<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01236(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->cookie('preferences');
    $prefs = json_decode($payload, true);
    return BenchmarkResponse::json($prefs ?? []);
}
