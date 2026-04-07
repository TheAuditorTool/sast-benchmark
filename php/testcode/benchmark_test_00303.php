<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00303(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['trim', 'strtolower', 'strtoupper', 'strlen'];
    $idx = (int) $req->param('idx');
    $fn = $allowed[$idx] ?? 'trim';
    $result = $fn($req->param('value'));
    return BenchmarkResponse::ok((string) $result);
}
