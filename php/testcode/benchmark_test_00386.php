<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00386(BenchmarkRequest $req): BenchmarkResponse {
    $field = $req->param('field');
    if (!preg_match('/^field_[0-9]+$/', $field)) return BenchmarkResponse::badRequest('invalid');
    $$field = $req->param('val');
    return BenchmarkResponse::ok('set');
}
