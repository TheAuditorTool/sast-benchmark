<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00343(BenchmarkRequest $req): BenchmarkResponse {
    $token = uniqid();
    setcookie('csrf', $token);
    return BenchmarkResponse::ok('csrf set');
}
