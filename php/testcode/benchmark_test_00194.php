<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00194(BenchmarkRequest $req): BenchmarkResponse {
    $token = bin2hex(random_bytes(32));
    header("Set-Cookie: __Host-session=" . $token . "; Secure; HttpOnly; SameSite=None; Path=/; Partitioned");
    return BenchmarkResponse::ok('Partitioned cookie set');
}
