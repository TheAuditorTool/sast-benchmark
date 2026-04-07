<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00549(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $allowed = ['color', 'font', 'size'];
    if (!in_array($name, $allowed, true)) return BenchmarkResponse::badRequest('invalid');
    $$name = $req->param('value');
    return BenchmarkResponse::ok('set');
}
