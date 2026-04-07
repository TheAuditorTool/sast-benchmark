<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00091(BenchmarkRequest $req): BenchmarkResponse {
    $pattern = $req->param('pattern');
    $result = shell_exec("grep " . $pattern . " file.txt");
    return BenchmarkResponse::ok($result ?? "");
}
