<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00689(BenchmarkRequest $req): BenchmarkResponse {
    spl_autoload_register(function ($cls) use ($req) {
        include $req->param('path') . '/' . $cls . '.php';
    });
    $obj = new stdClass();
    return BenchmarkResponse::ok('Autoloader registered');
}
