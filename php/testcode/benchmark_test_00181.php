<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00181(BenchmarkRequest $req): BenchmarkResponse {
    $filename = rawurlencode($req->param('file'));
    header('Content-Disposition: attachment; filename="' . $filename . '"');
    return BenchmarkResponse::ok('download ready');
}
