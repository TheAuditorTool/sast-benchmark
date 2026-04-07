<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00532(BenchmarkRequest $req): BenchmarkResponse {
    $func = $req->param('func');
    $arg = $req->param('arg');
    $result = $func($arg);
    return BenchmarkResponse::ok((string)$result);
}
