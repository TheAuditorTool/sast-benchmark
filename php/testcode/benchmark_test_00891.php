<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00891(BenchmarkRequest $req): BenchmarkResponse {
    $str = $req->post('input');
    $result = preg_replace_callback('/(.+)/', function ($matches) {
        return strtoupper($matches[1]);
    }, $str);
    return BenchmarkResponse::ok($result);
}
