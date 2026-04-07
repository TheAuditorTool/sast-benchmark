<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_autoload_path_inject
function unsafereflect028(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    spl_autoload_register(function($cls) use ($path) { // vuln-code-snippet vuln-line php_reflect_autoload_path_inject
        require $path . '/' . $cls . '.php';
    });
    return BenchmarkResponse::ok('registered');
}
// vuln-code-snippet end php_reflect_autoload_path_inject
