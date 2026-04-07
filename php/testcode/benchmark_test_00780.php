<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00780(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $content = file_get_contents("phar://" . $file);
    return BenchmarkResponse::ok($content);
}
