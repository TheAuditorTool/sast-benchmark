<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01095(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $val = constant($class . '::SECRET');
    return BenchmarkResponse::ok((string)$val);
}
