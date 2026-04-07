<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01247(BenchmarkRequest $req): BenchmarkResponse {
    $field = $req->param('field');
    $value = $req->param('value');
    $$field = $value;
    return BenchmarkResponse::ok('set');
}
