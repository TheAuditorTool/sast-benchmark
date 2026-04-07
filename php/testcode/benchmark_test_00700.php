<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00700(BenchmarkRequest $req): BenchmarkResponse {
    $lang = $req->cookie('lang');
    include $lang . '/strings.php';
    return BenchmarkResponse::ok('Strings loaded');
}
