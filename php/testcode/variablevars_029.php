<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_cookie_controlled_name
function variablevars029(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->cookie('name');
    $$name = $req->cookie('val'); // vuln-code-snippet vuln-line php_vv_cookie_controlled_name
    return BenchmarkResponse::ok('assigned');
}
// vuln-code-snippet end php_vv_cookie_controlled_name
