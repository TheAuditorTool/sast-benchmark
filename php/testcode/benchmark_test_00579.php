<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00579(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new stdClass();
    $prop = $req->post('prop');
    $val = $req->post('val');
    $obj->$prop = $val;
    return BenchmarkResponse::ok('written');
}
