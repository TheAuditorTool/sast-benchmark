<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01200(BenchmarkRequest $req): BenchmarkResponse {
    $module = $req->param('module');
    $allowed = ['dashboard', 'reports', 'settings'];
    if (!in_array($module, $allowed, true)) {
        return BenchmarkResponse::badRequest('unknown module');
    }
    require_once('/app/modules/' . $module . '.php');
    return BenchmarkResponse::ok('loaded');
}
