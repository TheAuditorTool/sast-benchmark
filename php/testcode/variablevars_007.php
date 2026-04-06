<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_foreach_post
function variablevars007(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $role = 'guest';
    foreach ($req->postData as $key => $value) {
        $$key = $value; // vuln-code-snippet vuln-line php_vv_foreach_post
    }
    return BenchmarkResponse::json(['admin' => $is_admin, 'role' => $role]);
}
// vuln-code-snippet end php_vv_foreach_post
