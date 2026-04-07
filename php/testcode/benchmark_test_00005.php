<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00005(BenchmarkRequest $req): BenchmarkResponse {
    $qs = http_build_query(['q' => $req->param('q')]);
    $url = 'https://api.example.com/search?' . $qs;
    $content = file_get_contents($url);
    return BenchmarkResponse::ok($content);
}
