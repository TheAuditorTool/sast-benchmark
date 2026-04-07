<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00298(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}
