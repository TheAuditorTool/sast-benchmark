<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00600(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $fp = fopen($url, 'r');
    $content = stream_get_contents($fp);
    fclose($fp);
    return BenchmarkResponse::ok($content);
}
