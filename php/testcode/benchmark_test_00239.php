<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00239(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = realpath(__DIR__ . '/templates');
    $file = $req->param('template');
    $resolved = realpath($baseDir . '/' . $file);
    if ($resolved === false || !str_starts_with($resolved, $baseDir)) {
        return BenchmarkResponse::badRequest("invalid template path");
    }
    include($resolved);
    return BenchmarkResponse::ok("template loaded");
}
