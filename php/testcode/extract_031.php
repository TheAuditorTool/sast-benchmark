<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_cookie_multihop
function extract031(BenchmarkRequest $req): BenchmarkResponse {
    $prefs = $req->cookie('p');
    parse_str($prefs); // vuln-code-snippet vuln-line php_extract_cookie_multihop
    if (!empty($isAdmin)) {
        return BenchmarkResponse::ok('Admin panel');
    }
    return BenchmarkResponse::ok('User panel');
}
// vuln-code-snippet end php_extract_cookie_multihop
