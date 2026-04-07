<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00151(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $value = $req->post('value');
    setcookie($name, $value);
    return BenchmarkResponse::ok('cookie set');
}
