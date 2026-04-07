<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00430(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('input');
    $result = preg_replace('/[aeiou]/', '*', $input);
    return BenchmarkResponse::ok((string) $result);
}
