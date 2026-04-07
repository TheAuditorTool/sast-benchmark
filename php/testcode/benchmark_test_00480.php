<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00480(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('password');
    $hash = '$2y$10$abcdefghijklmnopqrstuuABCDEFGHIJKLMNOPQRSTUVWXYZ01234';
    if (password_verify($input, $hash)) {
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::error('denied', 403);
}
