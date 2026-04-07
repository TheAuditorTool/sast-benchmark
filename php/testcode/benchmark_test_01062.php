<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01062(BenchmarkRequest $req): BenchmarkResponse {
    $pattern = $req->param('pattern');
    $files   = glob('templates/' . $pattern . '.php');
    include $files[0];
    return BenchmarkResponse::ok('Template rendered');
}
