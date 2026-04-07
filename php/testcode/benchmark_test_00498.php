<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00498(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    header("Set-Cookie: token=" . $token . "; Secure; HttpOnly; SameSite=Strict; Path=/");
    return BenchmarkResponse::ok('Token set');
}
