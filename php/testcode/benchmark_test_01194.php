<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01194(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $result = [];
    exec('convert /uploads/' . $filename . ' /thumbnails/' . $filename, $result);
    return BenchmarkResponse::ok(implode("\n", $result));
}
