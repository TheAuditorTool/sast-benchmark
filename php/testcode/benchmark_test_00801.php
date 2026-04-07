<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00801(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('next');
    header("Location: " . $url);
    return BenchmarkResponse::ok('');
}
