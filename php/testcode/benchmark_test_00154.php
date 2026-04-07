<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00154(BenchmarkRequest $req): BenchmarkResponse {
    $typeMap = ['json' => 'application/json', 'xml' => 'text/xml'];
    $ct = $typeMap[$req->param('type')] ?? 'application/octet-stream';
    header('Content-Type: ' . $ct);
    return BenchmarkResponse::ok('content type set');
}
