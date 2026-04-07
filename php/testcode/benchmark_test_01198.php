<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01198(BenchmarkRequest $req): BenchmarkResponse {
    $module = $req->param('module');
    require_once('/app/modules/' . $module . '.php');
    return BenchmarkResponse::ok('loaded');
}
