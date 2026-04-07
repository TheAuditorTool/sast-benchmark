<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_curly_post_assign
function variablevars020(BenchmarkRequest $req): BenchmarkResponse {
    $field = $req->post('field');
    $value = $req->post('value');
    ${$field} = $value; // vuln-code-snippet vuln-line php_vv_curly_post_assign
    return BenchmarkResponse::ok('assigned');
}
// vuln-code-snippet end php_vv_curly_post_assign
