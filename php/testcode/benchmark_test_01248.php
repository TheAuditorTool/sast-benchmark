<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01248(BenchmarkRequest $req): BenchmarkResponse {
    $field = $req->param('field');
    $value = $req->param('value');
    $store = [];
    $store[$field] = $value;
    return BenchmarkResponse::ok('set');
}
