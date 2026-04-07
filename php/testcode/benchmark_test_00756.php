<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00756(BenchmarkRequest $req): BenchmarkResponse {
    $module   = $req->param('module');
    $allowed  = ['auth', 'dashboard', 'reports'];
    if (!in_array($module, $allowed, true)) {
        return BenchmarkResponse::badRequest('Invalid module');
    }
    include constant('APP_PATH') . '/modules/' . $module . '/init.php';
    return BenchmarkResponse::ok('Module loaded');
}
