<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00020(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    $arg = $req->param('arg');
    $allowed = ['ls', 'pwd', 'date'];
    if (!in_array($cmd, $allowed, true)) {
        return BenchmarkResponse::badRequest('not allowed');
    }
    $output = [];
    exec(escapeshellarg($cmd) . ' ' . escapeshellarg($arg), $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
