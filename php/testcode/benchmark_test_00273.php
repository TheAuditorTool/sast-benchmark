<?php
require_once __DIR__ . '/shared.php';

define('CACHED_CONFIG', 'a:2:{s:4:"host";s:9:"localhost";s:4:"port";i:3306;}');

function benchmarkTest00273(BenchmarkRequest $req): BenchmarkResponse {
    $config = unserialize(CACHED_CONFIG);
    return BenchmarkResponse::json($config);
}
