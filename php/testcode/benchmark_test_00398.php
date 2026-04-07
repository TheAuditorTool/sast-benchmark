<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00398(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    http_response_code(301);
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
