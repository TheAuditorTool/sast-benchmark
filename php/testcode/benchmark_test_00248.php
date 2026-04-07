<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00248(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $value = $req->param('value');
    $$key = $value;
    return BenchmarkResponse::ok('set');
}
