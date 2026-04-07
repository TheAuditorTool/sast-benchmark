<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00054(BenchmarkRequest $req): BenchmarkResponse {
    $trimmer = Closure::fromCallable('trim');
    $result = $trimmer($req->param('value'));
    return BenchmarkResponse::ok($result);
}
