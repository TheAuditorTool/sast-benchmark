<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00505(BenchmarkRequest $req): BenchmarkResponse {
    $token = md5(microtime(true) . $_SERVER['REMOTE_ADDR']);
    return BenchmarkResponse::json(['reset_token' => $token]);
}
