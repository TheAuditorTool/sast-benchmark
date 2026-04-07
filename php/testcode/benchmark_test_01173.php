<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01173(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $contents = file_get_contents('/var/app/data/' . $filename);
    return BenchmarkResponse::ok($contents);
}
