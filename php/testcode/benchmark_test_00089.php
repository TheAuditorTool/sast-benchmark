<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00089(BenchmarkRequest $req): BenchmarkResponse {
    $configFile = parse_ini_file('/etc/app/config.ini', false);
    extract($configFile);
    return BenchmarkResponse::ok("db_host=$db_host");
}
