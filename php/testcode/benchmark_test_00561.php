<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00561(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    unlink("/tmp/" . $filename);
    return BenchmarkResponse::ok("File deleted");
}
