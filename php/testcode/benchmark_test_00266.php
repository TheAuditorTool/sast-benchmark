<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00266(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['ls', 'whoami', 'date', 'uptime'];
    $index = (int) $req->param('cmd_index');
    if ($index < 0 || $index >= count($allowed)) {
        return BenchmarkResponse::badRequest("invalid command index");
    }
    $output = [];
    exec($allowed[$index], $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
