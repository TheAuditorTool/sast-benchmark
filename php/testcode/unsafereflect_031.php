<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_full_dynamic_dispatch
function unsafereflect031(BenchmarkRequest $req): BenchmarkResponse {
    $class = $req->param('class');
    $method = $req->param('method');
    call_user_func([$class, $method]); // vuln-code-snippet vuln-line php_reflect_full_dynamic_dispatch
    return BenchmarkResponse::ok('dispatched');
}
// vuln-code-snippet end php_reflect_full_dynamic_dispatch
