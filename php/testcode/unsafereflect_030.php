<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_class_suffix_dispatch
function unsafereflect030(BenchmarkRequest $req): BenchmarkResponse {
    $suffix = $req->param('type');
    $className = 'Handler' . $suffix;
    $obj = new $className(); // vuln-code-snippet vuln-line php_reflect_class_suffix_dispatch
    return BenchmarkResponse::ok('dispatched');
}
// vuln-code-snippet end php_reflect_class_suffix_dispatch
