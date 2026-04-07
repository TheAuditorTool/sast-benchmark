<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_forward_static_both
function unsafereflect021(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $method = $req->param('method');
    forward_static_call([$class, $method]); // vuln-code-snippet vuln-line php_reflect_forward_static_both
    return BenchmarkResponse::ok('dispatched');
}
// vuln-code-snippet end php_reflect_forward_static_both
