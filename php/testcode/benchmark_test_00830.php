<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00830(BenchmarkRequest $req): BenchmarkResponse {
    $fn = Closure::fromCallable('htmlspecialchars');
    $result = $fn($req->param('input'));
    return BenchmarkResponse::ok($result);
}
