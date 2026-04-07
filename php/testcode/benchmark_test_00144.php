<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00144(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('field');
    $value = $req->param('value');
    $obj = new stdClass();
    $obj->{$prop} = $value;
    return BenchmarkResponse::json((array) $obj);
}
