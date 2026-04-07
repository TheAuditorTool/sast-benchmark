<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_object_dynamic_prop
function variablevars021(BenchmarkRequest $req): BenchmarkResponse {
    $obj = new stdClass();
    $prop = $req->post('prop');
    $val = $req->post('val');
    $obj->$prop = $val; // vuln-code-snippet vuln-line php_vv_object_dynamic_prop
    return BenchmarkResponse::ok('written');
}
// vuln-code-snippet end php_vv_object_dynamic_prop
