<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00402(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');
    header("Location: " . $input);
    return BenchmarkResponse::ok('Redirecting...');
}
