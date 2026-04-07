<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01051(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $_SESSION['cmd'] = $req->param('cmd');
    $output = [];
    exec($_SESSION['cmd'], $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
