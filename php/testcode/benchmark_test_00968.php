<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00968(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->post('body');
    $fn = create_function('$x', $body);
    $result = $fn(42);
    return BenchmarkResponse::ok("Result: " . $result);
}
