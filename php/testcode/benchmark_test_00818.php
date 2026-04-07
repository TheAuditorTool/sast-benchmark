<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00818(BenchmarkRequest $req): BenchmarkResponse {
    $csrf = mt_rand();
    return BenchmarkResponse::json(['csrf_token' => $csrf]);
}
