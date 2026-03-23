<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_property_allowlist
function variablevars004(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('field');
    $value = $req->param('value');
    $allowed = ['name', 'email', 'bio'];
    if (!in_array($prop, $allowed, true)) { // vuln-code-snippet safe-line php_vv_property_allowlist
        return BenchmarkResponse::badRequest('invalid field');
    }
    $obj = new stdClass();
    $obj->{$prop} = $value;
    return BenchmarkResponse::json((array) $obj);
}
// vuln-code-snippet end php_vv_property_allowlist
