<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00671(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $out = `ls $userPath`;
    return BenchmarkResponse::ok($out ?? '');
}
