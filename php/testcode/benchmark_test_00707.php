<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00707(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $content = file_get_contents("/uploads/" . $file);
    return BenchmarkResponse::ok($content);
}
