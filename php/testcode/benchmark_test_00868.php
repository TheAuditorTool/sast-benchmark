<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00868(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    spl_autoload_register(function($cls) use ($path) {
        require $path . '/' . $cls . '.php';
    });
    return BenchmarkResponse::ok('registered');
}
