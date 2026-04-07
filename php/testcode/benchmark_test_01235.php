<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01235(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->cookie('preferences');
    $prefs = unserialize($payload);
    return BenchmarkResponse::json((array)$prefs);
}
