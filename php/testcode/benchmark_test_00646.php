<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00646(BenchmarkRequest $req): BenchmarkResponse {
    $funcname = $req->param('action');
    $result = $$funcname();
    return BenchmarkResponse::ok((string)$result);
}
