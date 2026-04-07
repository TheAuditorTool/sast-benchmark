<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00157(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    $safe = basename($filename);
    header("Content-Disposition: attachment; filename=\"" . $safe . "\"");
    return BenchmarkResponse::ok('File download');
}
