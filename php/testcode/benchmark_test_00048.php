<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00048(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->param('code');
    $str = 'hello world';
    $result = preg_replace_callback('/(.+)/', function ($m) use ($code) {
        return eval($code);
    }, $str);
    return BenchmarkResponse::ok((string) $result);
}
