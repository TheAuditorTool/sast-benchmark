<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00372(BenchmarkRequest $req): BenchmarkResponse {
    $module = $req->param('module');
    include $module;
    return BenchmarkResponse::ok('Module loaded');
}
