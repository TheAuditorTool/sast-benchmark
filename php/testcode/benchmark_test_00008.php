<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00008(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    $safe = basename($filename);
    unlink("/tmp/" . $safe);
    return BenchmarkResponse::ok("File deleted");
}
