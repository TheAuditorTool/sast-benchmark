<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00148(BenchmarkRequest $req): BenchmarkResponse {
    $source = $req->param('source');
    $dest = $req->param('dest');
    copy($source, $dest);
    return BenchmarkResponse::ok("File copied");
}
