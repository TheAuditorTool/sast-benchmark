<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01121(BenchmarkRequest $req): BenchmarkResponse {
    $expr = $req->param('expr');
    assert($expr);
    return BenchmarkResponse::ok("Assertion passed");
}
