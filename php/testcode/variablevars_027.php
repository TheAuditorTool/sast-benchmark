<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_dynamic_prop_read_object
function variablevars027(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $obj = new stdClass();
    $obj->secret = 'api_key_123';
    echo $obj->$prop; // vuln-code-snippet vuln-line php_vv_dynamic_prop_read_object
    return BenchmarkResponse::ok('read');
}
// vuln-code-snippet end php_vv_dynamic_prop_read_object
