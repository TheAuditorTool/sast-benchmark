<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00947(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->param('action');
    $allowed = ['strlen', 'strtoupper', 'strtolower', 'trim'];
    if (!in_array($action, $allowed, true)) {
        return BenchmarkResponse::badRequest('Function not allowed');
    }
    $fn = Closure::fromCallable($action);
    $result = $fn($req->param('input'));
    return BenchmarkResponse::ok((string)$result);
}
