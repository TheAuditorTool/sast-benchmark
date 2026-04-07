<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01122(BenchmarkRequest $req): BenchmarkResponse {
    $configKeys = parse_ini_file('/etc/app/display.ini');
    foreach ($configKeys as $k => $v) {
        $$k = $v;
    }
    return BenchmarkResponse::ok('loaded');
}
