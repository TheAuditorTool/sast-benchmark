<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00884(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('input');
    $result = preg_replace('/(.+)/e', '$1', $input);
    return BenchmarkResponse::ok((string) $result);
}
