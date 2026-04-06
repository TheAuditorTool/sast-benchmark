<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_json_body
function extract011(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $data = json_decode($req->bodyStr(), true);
    extract($data); // vuln-code-snippet vuln-line php_extract_json_body
    if ($is_admin) {
        return BenchmarkResponse::ok('admin panel');
    }
    return BenchmarkResponse::ok('user panel');
}
// vuln-code-snippet end php_extract_json_body
