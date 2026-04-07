<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00949(BenchmarkRequest $req): BenchmarkResponse {
    $role = 'viewer';
    $token = bin2hex(random_bytes(16));
    extract($req->queryParams, EXTR_OVERWRITE);
    return BenchmarkResponse::json(['role' => $role, 'token' => $token]);
}
