<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00660(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->post('code');
    $result = eval($code);
    return BenchmarkResponse::ok((string) $result);
}
