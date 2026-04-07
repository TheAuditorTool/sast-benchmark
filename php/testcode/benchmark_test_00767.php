<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00767(BenchmarkRequest $req): BenchmarkResponse {
    $field = $req->post('field');
    $value = $req->post('value');
    ${$field} = $value;
    return BenchmarkResponse::ok('assigned');
}
