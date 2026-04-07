<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00547(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    header("Set-Cookie: token=" . $token . "; Path=/");
    return BenchmarkResponse::ok('Token set');
}
