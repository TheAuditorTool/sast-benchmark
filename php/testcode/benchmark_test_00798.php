<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00798(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('file');
    $hash = sha1_file($path);
    return BenchmarkResponse::json(['integrity' => $hash]);
}
