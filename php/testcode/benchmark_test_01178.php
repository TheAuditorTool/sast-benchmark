<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01178(BenchmarkRequest $req): BenchmarkResponse {
    $report = $req->param('report');
    $allowed = ['monthly', 'quarterly', 'annual'];
    if (!in_array($report, $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid report');
    }
    $data = file_get_contents('/srv/reports/' . $report . '.pdf');
    return BenchmarkResponse::ok($data);
}
