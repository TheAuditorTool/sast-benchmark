<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_new_class
function unsafereflect001(BenchmarkRequest $req): BenchmarkResponse {
    $className = $req->param('handler');
    $obj = new $className(); // vuln-code-snippet vuln-line php_reflect_new_class
    $result = $obj->handle();
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_new_class
