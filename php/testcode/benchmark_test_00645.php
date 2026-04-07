<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00645(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = realpath('/var/app/files');
    $input = $req->param('file');
    $real = realpath($baseDir . '/' . $input);
    if ($real === false || strpos($real, $baseDir) !== 0) {
        return BenchmarkResponse::badRequest('access denied');
    }
    $content = file_get_contents($real);
    return BenchmarkResponse::ok($content);
}
