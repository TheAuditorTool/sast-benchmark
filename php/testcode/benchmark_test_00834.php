<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00834(BenchmarkRequest $req): BenchmarkResponse {
    $admin = $req->param('admin');
    ${!$admin} = true;
    return BenchmarkResponse::ok('done');
}
