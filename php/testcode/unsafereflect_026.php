<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_property_write
function unsafereflect026(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $val = $req->param('val');
    $obj = new stdClass();
    if (property_exists($obj, $prop)) {
        $obj->$prop = $val; // vuln-code-snippet vuln-line php_reflect_property_write
    }
    return BenchmarkResponse::ok('written');
}
// vuln-code-snippet end php_reflect_property_write
