<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01101(BenchmarkRequest $req): BenchmarkResponse {
    spl_autoload_register(function (string $cls): void {
        $file = __DIR__ . '/src/' . str_replace('\\', '/', $cls) . '.php';
        if (is_file($file)) {
            include $file;
        }
    });
    return BenchmarkResponse::ok('Autoloader registered');
}
