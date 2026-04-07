<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00899(BenchmarkRequest $req): BenchmarkResponse {
    $module = $req->param('mod');
    if (!ctype_alnum($module)) {
        return BenchmarkResponse::badRequest('Invalid module name');
    }
    include __DIR__ . '/modules/' . $module . '/init.php';
    return BenchmarkResponse::ok('Module loaded');
}
