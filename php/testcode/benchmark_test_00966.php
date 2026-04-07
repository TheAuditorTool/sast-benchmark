<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00966(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['/usr/bin/ls', '/usr/bin/whoami', '/usr/bin/date'];
    $path = $req->param('executable');
    if (!in_array($path, $allowed, true)) {
        return BenchmarkResponse::badRequest("executable not allowed");
    }
    pcntl_exec($path);
    return BenchmarkResponse::error("exec failed");
}
