<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00528(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    $descriptors = [1 => ['pipe', 'w'], 2 => ['pipe', 'w']];
    $proc = proc_open($cmd, $descriptors, $pipes);
    $output = stream_get_contents($pipes[1]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    proc_close($proc);
    return BenchmarkResponse::ok($output);
}
