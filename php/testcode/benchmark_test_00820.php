<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00820(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $path = 'php://filter/read=convert.base64-encode/resource=' . $file;
    include $path;
    return BenchmarkResponse::ok('Done');
}
