<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01097(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $val = $req->param('val');
    $obj = new stdClass();
    if (property_exists($obj, $prop)) {
        $obj->$prop = $val;
    }
    return BenchmarkResponse::ok('written');
}
