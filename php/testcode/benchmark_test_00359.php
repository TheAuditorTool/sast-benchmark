<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00359(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('field');
    $value = $req->param('value');
    $allowed = ['name', 'email', 'bio'];
    if (!in_array($prop, $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid field');
    }
    $obj = new stdClass();
    $obj->{$prop} = $value;
    return BenchmarkResponse::json((array) $obj);
}
