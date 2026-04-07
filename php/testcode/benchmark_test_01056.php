<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01056(BenchmarkRequest $req): BenchmarkResponse {
    $name  = 'Alice';
    $email = 'alice@example.com';
    $role  = 'admin';
    $varNames = $$req->param('vars');
    $result = compact($varNames);
    return BenchmarkResponse::json($result);
}
