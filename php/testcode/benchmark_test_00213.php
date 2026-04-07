<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00213(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    header("Content-Disposition: attachment; filename=\"" . $filename . "\"");
    return BenchmarkResponse::ok(file_get_contents('/tmp/exports/report.csv'));
}
