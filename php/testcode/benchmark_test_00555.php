<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00555(BenchmarkRequest $req): BenchmarkResponse {
    $host = $req->param('host');
    $bin = inet_pton($host);
    if ($bin === false || (ord($bin[0]) & 0xFE) === 0xFE) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $content = file_get_contents('http://' . $host . '/data');
    return BenchmarkResponse::ok($content);
}
