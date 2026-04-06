<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_cookie_scope
function extract013(BenchmarkRequest $req): BenchmarkResponse {
    $authenticated = false;
    $user_role = 'anonymous';
    extract($req->cookies); // vuln-code-snippet vuln-line php_extract_cookie_scope
    if ($authenticated) {
        return BenchmarkResponse::ok('Welcome, ' . $user_role);
    }
    return BenchmarkResponse::ok('Please log in');
}
// vuln-code-snippet end php_extract_cookie_scope
