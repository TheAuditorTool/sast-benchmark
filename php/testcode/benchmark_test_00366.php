<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00366(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('type');
    $result = match($input) {
        '1' => 'admin',
        '2' => 'user',
        default => 'guest',
    };
    return BenchmarkResponse::ok($result);
}
