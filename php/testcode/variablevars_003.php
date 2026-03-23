<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_dynamic_property
function variablevars003(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('field');
    $value = $req->param('value');
    $obj = new stdClass();
    $obj->{$prop} = $value; // vuln-code-snippet vuln-line php_vv_dynamic_property
    return BenchmarkResponse::json((array) $obj);
}
// vuln-code-snippet end php_vv_dynamic_property
