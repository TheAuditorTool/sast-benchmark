<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_array_combine_allowlist
function variablevars013(BenchmarkRequest $req): BenchmarkResponse {
    $keys = ['name', 'email', 'phone'];
    $values = [
        $req->post('name'),
        $req->post('email'),
        $req->post('phone'),
    ];
    $data = array_combine($keys, $values); // vuln-code-snippet safe-line php_vv_array_combine_allowlist
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_vv_array_combine_allowlist
