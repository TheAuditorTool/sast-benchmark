<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00163(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('name');
    $value = 'active';
    setcookie($name, $value);
    return BenchmarkResponse::ok('Cookie set');
}
