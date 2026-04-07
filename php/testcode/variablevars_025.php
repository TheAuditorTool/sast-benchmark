<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_config_key_from_request
function variablevars025(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $value = $req->param('value');
    $$key = $value; // vuln-code-snippet vuln-line php_vv_config_key_from_request
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_vv_config_key_from_request
