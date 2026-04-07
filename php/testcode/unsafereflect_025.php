<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_static_both_dynamic
function unsafereflect025(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $method = $req->param('method');
    $class::$method(); // vuln-code-snippet vuln-line php_reflect_static_both_dynamic
    return BenchmarkResponse::ok('dispatched');
}
// vuln-code-snippet end php_reflect_static_both_dynamic
