<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01174(BenchmarkRequest $req): BenchmarkResponse {
    $report = $req->param('report');
    $path = '/srv/reports/' . $report . '.pdf';
    $data = file_get_contents($path);
    return BenchmarkResponse::ok($data);
}
