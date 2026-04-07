<?php
require_once __DIR__ . '/shared.php';

class DbConnection {}

function benchmarkTest00558(BenchmarkRequest $req): BenchmarkResponse {
    $container = new stdClass();
    $container->bind = function(string $iface, string $concrete) {};
    $svc = new UserService(new DbConnection());
    return BenchmarkResponse::ok('resolved');
}
