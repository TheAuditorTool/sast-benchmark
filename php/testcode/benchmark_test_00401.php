<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00401(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $val = $req->param('val');
    $allowed = ['lang', 'theme'];
    if (!in_array($prop, $allowed, true)) return BenchmarkResponse::badRequest('invalid');
    $obj = new stdClass();
    $obj->$prop = $val;
    return BenchmarkResponse::ok('set');
}
