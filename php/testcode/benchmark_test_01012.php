<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01012(BenchmarkRequest $req): BenchmarkResponse {
    $origin = $req->header('Origin');
    $referer = $req->header('Referer');
    $allowedOrigin = 'https://app.example.com';
    $originOk = ($origin === $allowedOrigin);
    $refererOk = str_starts_with($referer, $allowedOrigin . '/');
    if (!$originOk && !$refererOk) {
        return BenchmarkResponse::badRequest('Origin validation failed');
    }
    return BenchmarkResponse::ok('Action performed');
}
