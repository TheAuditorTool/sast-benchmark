<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00031(BenchmarkRequest $req): BenchmarkResponse {
    extract($_POST);
    $key = $req->param('key');
    echo $$key;
    return BenchmarkResponse::ok('read');
}
