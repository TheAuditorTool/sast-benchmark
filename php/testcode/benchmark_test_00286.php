<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00286(BenchmarkRequest $req): BenchmarkResponse {
    $url = urldecode($req->param('url'));
    header("Location: " . $url);
    return BenchmarkResponse::ok('');
}
