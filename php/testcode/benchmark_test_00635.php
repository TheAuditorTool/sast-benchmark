<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00635(BenchmarkRequest $req): BenchmarkResponse {
    $version = $req->param('version');
    define('APP_VERSION', $version);
    return BenchmarkResponse::ok('Version set to ' . APP_VERSION);
}
