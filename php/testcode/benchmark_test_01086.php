<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01086(BenchmarkRequest $req): BenchmarkResponse {
    $lang = $req->param('lang');
    require $lang . '.php';
    return BenchmarkResponse::ok('Loaded');
}
