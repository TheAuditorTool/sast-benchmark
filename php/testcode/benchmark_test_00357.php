<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00357(BenchmarkRequest $req): BenchmarkResponse {
    $ops = ['trim' => fn($s) => trim($s), 'upper' => fn($s) => strtoupper($s)];
    $op = $ops[$req->param('op')] ?? null;
    if (!$op) return BenchmarkResponse::badRequest('invalid');
    $result = $op($req->param('input'));
    return BenchmarkResponse::ok($result);
}
