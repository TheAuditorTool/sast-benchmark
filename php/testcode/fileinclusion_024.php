<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_autoload_path_inject
function fileinclusion024(BenchmarkRequest $req): BenchmarkResponse {
    spl_autoload_register(function ($cls) use ($req) {
        include $req->param('path') . '/' . $cls . '.php'; // vuln-code-snippet vuln-line php_fi_autoload_path_inject
    });
    $obj = new stdClass();
    return BenchmarkResponse::ok('Autoloader registered');
}
// vuln-code-snippet end php_fi_autoload_path_inject
