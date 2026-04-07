<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00993(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = "/uploads";
    $filename = $req->param('filename');
    $resolved = realpath($baseDir . "/" . $filename);
    if ($resolved === false || !str_starts_with($resolved, $baseDir)) {
        return BenchmarkResponse::badRequest("Invalid path");
    }
    $content = file_get_contents($resolved);
    return BenchmarkResponse::ok($content);
}
