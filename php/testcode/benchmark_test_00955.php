<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00955(BenchmarkRequest $req): BenchmarkResponse {
    $jwt = 'eyJhbGc...fake';
    header('Authorization: Bearer ' . $jwt);
    return BenchmarkResponse::ok('jwt in header');
}
