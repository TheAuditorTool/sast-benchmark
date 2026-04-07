<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00534(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $headers = get_headers($url);
    return BenchmarkResponse::json($headers);
}
