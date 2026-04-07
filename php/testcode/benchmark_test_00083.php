<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00083(BenchmarkRequest $req): BenchmarkResponse {
    $dest = $req->param('dest');
    header("Location: " . $dest);
    return BenchmarkResponse::ok('');
}
