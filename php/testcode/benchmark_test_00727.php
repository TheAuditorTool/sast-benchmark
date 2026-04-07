<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00727(BenchmarkRequest $req): BenchmarkResponse {
    $jwt = 'eyJhbGc...fake.jwt.token';
    setcookie('jwt', $jwt, time() + 3600, '/', '', true, false);
    return BenchmarkResponse::ok('jwt set');
}
