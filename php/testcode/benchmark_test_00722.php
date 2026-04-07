<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00722(BenchmarkRequest $req): BenchmarkResponse {
    $target = '/var/app/public/shared.css';
    $userLink = $req->param('link');
    symlink($target, $userLink);
    return BenchmarkResponse::ok('linked');
}
