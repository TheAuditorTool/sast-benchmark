<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00817(BenchmarkRequest $req): BenchmarkResponse {
    $ops = [
        'upper' => fn($s) => strtoupper($s),
        'lower' => fn($s) => strtolower($s),
        'trim'  => fn($s) => trim($s),
    ];
    $key = $req->param('op');
    $fn = isset($ops[$key]) ? $ops[$key] : $ops['trim'];
    $result = $fn($req->param('value'));
    return BenchmarkResponse::ok($result);
}
