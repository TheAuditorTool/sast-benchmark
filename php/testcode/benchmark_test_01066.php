<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01066(BenchmarkRequest $req): BenchmarkResponse {
    $prefix = $req->param('prefix');
    $allowedPrefixes = ['lang', 'theme', 'display'];
    if (!in_array($prefix, $allowedPrefixes, true)) return BenchmarkResponse::badRequest('invalid');
    $varName = $prefix . '_id';
    $$varName = intval($req->param('val'));
    return BenchmarkResponse::ok('set');
}
