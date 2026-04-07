<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00382(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $safe = basename($file);
    $content = file_get_contents("/uploads/" . $safe);
    return BenchmarkResponse::ok($content);
}
