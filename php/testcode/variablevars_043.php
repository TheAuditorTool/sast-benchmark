<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_property_accessor_allowlist
function variablevars043(BenchmarkRequest $req): BenchmarkResponse {
    $prop = $req->param('prop');
    $val = $req->param('val');
    $allowed = ['lang', 'theme'];
    if (!in_array($prop, $allowed, true)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_vv_property_accessor_allowlist
    $obj = new stdClass();
    $obj->$prop = $val;
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_property_accessor_allowlist
