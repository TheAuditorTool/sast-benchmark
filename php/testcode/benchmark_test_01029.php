<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01029(BenchmarkRequest $req): BenchmarkResponse {
    header('Set-Cookie: __Host-session=' . bin2hex(random_bytes(32)) . '; Secure; HttpOnly; SameSite=None; Path=/; Partitioned');
    return BenchmarkResponse::ok('partitioned cookie set');
}
