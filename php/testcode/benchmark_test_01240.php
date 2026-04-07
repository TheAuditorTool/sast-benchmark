<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01240(BenchmarkRequest $req): BenchmarkResponse {
    $dest = $req->param('location');
    $clean = str_replace(["\r", "\n", "\0"], '', $dest);
    header('Location: ' . $clean);
    return BenchmarkResponse::ok('redirecting');
}
