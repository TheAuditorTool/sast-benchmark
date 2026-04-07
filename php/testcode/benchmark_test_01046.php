<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01046(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    $safe = basename($tpl);
    $path = __DIR__ . '/templates/' . $safe . '.php';
    include $path;
    return BenchmarkResponse::ok('rendered');
}
