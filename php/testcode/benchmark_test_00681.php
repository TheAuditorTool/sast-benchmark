<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00681(BenchmarkRequest $req): BenchmarkResponse {
    $func = $req->param('func');
    $args = $req->param('args');
    $result = call_user_func($func, $args);
    return BenchmarkResponse::ok("Result: " . $result);
}
