<?php
require_once __DIR__ . '/shared.php';

function runCommand(string $cmd): string {
    $output = [];
    exec($cmd, $output);
    return implode("\n", $output);
}

function benchmarkTest00070(BenchmarkRequest $req): BenchmarkResponse {
    $userCmd = "ls " . $req->param('dir');
    $result = runCommand($userCmd);
    return BenchmarkResponse::ok($result);
}
