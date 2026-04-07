<?php
require_once __DIR__ . '/shared.php';

function sanitizeAndRun(string $arg): string {
    $output = [];
    exec("ls " . escapeshellarg($arg), $output);
    return implode("\n", $output);
}

function benchmarkTest00351(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    $result = sanitizeAndRun($dir);
    return BenchmarkResponse::ok($result);
}
