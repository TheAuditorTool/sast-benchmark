<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01159(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    return BenchmarkResponse::html('<h1>Hello, ' . $name . '</h1>');
}
