<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00323(BenchmarkRequest $req): BenchmarkResponse {
    $filename = basename($req->param('file'));
    $path = '/var/app/files/' . $filename;
    if (is_link($path)) {
        return BenchmarkResponse::badRequest('symlinks not allowed');
    }
    $content = file_get_contents($path);
    return BenchmarkResponse::ok($content);
}
