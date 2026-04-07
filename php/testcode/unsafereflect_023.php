<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_class_exists_no_guard
function unsafereflect023(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('class');
    if (class_exists($input)) {
        $obj = new $input(); // vuln-code-snippet vuln-line php_reflect_class_exists_no_guard
    }
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_reflect_class_exists_no_guard
