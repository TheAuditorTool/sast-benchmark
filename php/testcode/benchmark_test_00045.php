<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00045(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $obj = new stdClass();
    $obj->secret = 'api_key_123';
    echo $obj->$prop;
    return BenchmarkResponse::ok('read');
}
