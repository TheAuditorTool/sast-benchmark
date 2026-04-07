<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00882(BenchmarkRequest $req): BenchmarkResponse {
    $handlers = [0 => 'trim', 1 => 'strtolower'];
    $id = intval($req->param('id'));
    $fn = $handlers[$id] ?? 'trim';
    $result = $fn($req->param('value'));
    return BenchmarkResponse::ok((string) $result);
}
