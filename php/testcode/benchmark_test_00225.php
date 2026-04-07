<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00225(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $allowed = ['click', 'submit', 'load'];
    if (!in_array($type, $allowed, true)) return BenchmarkResponse::badRequest('invalid');
    $obj = new stdClass();
    $obj->$type = true;
    return BenchmarkResponse::ok('registered');
}
