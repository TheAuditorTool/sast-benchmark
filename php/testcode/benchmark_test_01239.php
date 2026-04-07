<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01239(BenchmarkRequest $req): BenchmarkResponse {
    $dest = $req->param('location');
    header('Location: ' . $dest);
    return BenchmarkResponse::ok('redirecting');
}
