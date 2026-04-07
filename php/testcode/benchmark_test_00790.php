<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00790(BenchmarkRequest $req): BenchmarkResponse {
    $sid = $req->param('sid');
    header('Set-Cookie: session=' . $sid . '; Path=/');
    return BenchmarkResponse::ok('session set');
}
