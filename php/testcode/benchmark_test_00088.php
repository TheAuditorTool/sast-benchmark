<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00088(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->param('code');
    $stream = 'data://text/plain;base64,' . base64_encode($code);
    ob_start();
    include $stream;
    $output = ob_get_clean();
    return BenchmarkResponse::ok($output);
}
