<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00140(BenchmarkRequest $req): BenchmarkResponse {
    $replacement = $req->post('replacement');
    $str = $req->post('input');
    $result = preg_replace('/(.+)/e', $replacement, $str);
    return BenchmarkResponse::ok($result);
}
