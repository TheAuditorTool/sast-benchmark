<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00288(BenchmarkRequest $req): BenchmarkResponse {
    $expr = $req->param('expr');
    $result = assert($expr);
    return BenchmarkResponse::ok($result ? 'true' : 'false');
}
