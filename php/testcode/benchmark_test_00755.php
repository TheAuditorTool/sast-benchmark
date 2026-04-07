<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00755(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->post('theme');
    setcookie('theme', $val, 0, $_SERVER['PHP_SELF']);
    return BenchmarkResponse::ok('Theme saved');
}
