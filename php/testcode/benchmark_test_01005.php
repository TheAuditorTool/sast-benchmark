<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01005(BenchmarkRequest $req): BenchmarkResponse {
    $userInput = $req->param('data');
    $descriptors = [0 => ['pipe', 'r'], 1 => ['pipe', 'w'], 2 => ['pipe', 'w']];
    $proc = proc_open("sort", $descriptors, $pipes);
    fwrite($pipes[0], $userInput);
    fclose($pipes[0]);
    $output = stream_get_contents($pipes[1]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    proc_close($proc);
    return BenchmarkResponse::ok($output);
}
