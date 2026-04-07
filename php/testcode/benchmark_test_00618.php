<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00618(BenchmarkRequest $req): BenchmarkResponse {
    $origin = $req->param('origin');
    header('Access-Control-Allow-Origin: ' . $origin);
    header('Access-Control-Allow-Credentials: true');
    return BenchmarkResponse::ok('cors set');
}
