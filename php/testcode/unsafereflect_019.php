<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_new_user_class
function unsafereflect019(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('class');
    $obj = new $className(); // vuln-code-snippet vuln-line php_reflect_new_user_class
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_reflect_new_user_class
