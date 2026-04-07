<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00542(BenchmarkRequest $req): BenchmarkResponse {
    $userDir = $req->param('dir');
    $handle = popen("tar czf backup.tar $userDir", 'r');
    $output = fread($handle, 4096);
    pclose($handle);
    return BenchmarkResponse::ok($output);
}
