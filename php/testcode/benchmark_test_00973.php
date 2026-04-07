<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00973(BenchmarkRequest $req): BenchmarkResponse {
    $b64 = $req->param('b64');
    include 'data://text/plain;base64,' . $b64;
    return BenchmarkResponse::ok('Done');
}
