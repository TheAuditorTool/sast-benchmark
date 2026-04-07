<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01042(BenchmarkRequest $req): BenchmarkResponse {
    $name = 'default';
    $email = 'none@example.com';
    extract($req->postData, EXTR_IF_EXISTS);
    return BenchmarkResponse::json(['name' => $name, 'email' => $email]);
}
