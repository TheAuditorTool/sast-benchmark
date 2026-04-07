<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00875(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->param('code');
    $fn = create_function('', $code);
    $result = $fn();
    return BenchmarkResponse::ok((string) $result);
}
