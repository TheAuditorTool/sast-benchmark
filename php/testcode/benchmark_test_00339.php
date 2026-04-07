<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00339(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    $output = [];
    exec(escapeshellcmd("ls") . " " . escapeshellarg($dir), $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
