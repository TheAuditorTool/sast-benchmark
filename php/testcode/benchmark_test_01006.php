<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01006(BenchmarkRequest $req): BenchmarkResponse {
    $file = basename($req->param('file'));
    readfile('/var/app/public/' . $file);
    return BenchmarkResponse::ok('');
}
