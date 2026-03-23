<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_parse_str
function extract005(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    parse_str($req->bodyStr()); // vuln-code-snippet vuln-line php_extract_parse_str
    if ($is_admin) {
        return BenchmarkResponse::ok('admin access');
    }
    return BenchmarkResponse::ok('user access');
}
// vuln-code-snippet end php_extract_parse_str
