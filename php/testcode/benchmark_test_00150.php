<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00150(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    ob_start();
    readfile($path);
    $content = ob_get_clean();
    return BenchmarkResponse::ok($content);
}
