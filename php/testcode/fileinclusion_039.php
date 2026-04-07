<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_psr4_autoloader
function fileinclusion039(BenchmarkRequest $req): BenchmarkResponse {
    spl_autoload_register(function (string $cls): void {
        $file = __DIR__ . '/src/' . str_replace('\\', '/', $cls) . '.php'; // vuln-code-snippet safe-line php_fi_psr4_autoloader
        if (is_file($file)) {
            include $file;
        }
    });
    return BenchmarkResponse::ok('Autoloader registered');
}
// vuln-code-snippet end php_fi_psr4_autoloader
