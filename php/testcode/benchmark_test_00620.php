<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00620(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('auth', bin2hex(random_bytes(16)), time() + 3600, '/', '.example.com', true, true);
    return BenchmarkResponse::ok('cookie set');
}
