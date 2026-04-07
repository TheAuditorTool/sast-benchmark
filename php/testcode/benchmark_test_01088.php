<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01088(BenchmarkRequest $req): BenchmarkResponse {
    $safePath = '/var/app/data';
    $descriptors = [0 => ['pipe','r'], 1 => ['pipe','w']];
    $proc = proc_open(['ls', '-la', $safePath], $descriptors, $pipes);
    $output = stream_get_contents($pipes[1]);
    proc_close($proc);
    return BenchmarkResponse::ok($output);
}
