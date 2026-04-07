<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00068(BenchmarkRequest $req): BenchmarkResponse {
    $qs = $_SERVER['QUERY_STRING'];
    parse_str($qs, $GLOBALS);
    return BenchmarkResponse::ok('Parsed into globals');
}
