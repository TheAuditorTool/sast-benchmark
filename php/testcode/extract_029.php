<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_admin_flag_inject
function extract029(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $opts = $req->param('opts');
    parse_str($opts); // vuln-code-snippet vuln-line php_extract_admin_flag_inject
    if ($is_admin) {
        return BenchmarkResponse::ok('Admin access granted');
    }
    return BenchmarkResponse::ok('Regular user');
}
// vuln-code-snippet end php_extract_admin_flag_inject
