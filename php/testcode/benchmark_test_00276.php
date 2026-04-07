<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00276(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->param('username');
    header("X-User: " . $username);
    return BenchmarkResponse::ok('Header set');
}
