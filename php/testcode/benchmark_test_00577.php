<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00577(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    header("Location: " . $url);
    return BenchmarkResponse::ok('Redirecting...');
}
