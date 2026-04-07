<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00808(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = "/var/www/files";
    $source = realpath($baseDir . "/" . $req->param('source'));
    $dest = realpath(dirname($baseDir . "/" . $req->param('dest'))) . "/" . basename($req->param('dest'));
    if ($source === false || !str_starts_with($source, $baseDir)) {
        return BenchmarkResponse::badRequest("Invalid source path");
    }
    if (!str_starts_with($dest, $baseDir)) {
        return BenchmarkResponse::badRequest("Invalid dest path");
    }
    copy($source, $dest);
    return BenchmarkResponse::ok("File copied");
}
