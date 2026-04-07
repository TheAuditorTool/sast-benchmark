<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00405(BenchmarkRequest $req): BenchmarkResponse {
    $allowedFuncs = ['strlen', 'strtolower', 'trim'];
    $func = $req->param('func');
    $args = $req->param('args');
    if (!in_array($func, $allowedFuncs, true)) {
        return BenchmarkResponse::badRequest("Function not allowed");
    }
    $result = call_user_func($func, $args);
    return BenchmarkResponse::ok("Result: " . $result);
}
