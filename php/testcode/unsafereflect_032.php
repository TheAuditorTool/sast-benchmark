<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_interface_from_post
function unsafereflect032(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->post('type');
    $obj = new $type(); // vuln-code-snippet vuln-line php_reflect_interface_from_post
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_reflect_interface_from_post
