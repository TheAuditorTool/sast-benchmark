<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_allowlisted_event_types
function variablevars047(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    $allowed = ['click', 'submit', 'load'];
    if (!in_array($type, $allowed, true)) return BenchmarkResponse::badRequest('invalid'); // vuln-code-snippet safe-line php_vv_allowlisted_event_types
    $obj = new stdClass();
    $obj->$type = true;
    return BenchmarkResponse::ok('registered');
}
// vuln-code-snippet end php_vv_allowlisted_event_types
