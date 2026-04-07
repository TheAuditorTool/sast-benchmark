<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00559(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->param('username');
    header("X-User: " . urlencode($username));
    return BenchmarkResponse::ok('Header set');
}
