<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00974(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('code');
    $result = preg_match('/^[a-z]+$/', $input);
    if ($result == 0) {
        return BenchmarkResponse::ok('no match');
    }
    return BenchmarkResponse::ok('match');
}
