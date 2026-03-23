<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_overwrite_auth
function extract007(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $auth_token = null;
    extract($req->queryParams); // vuln-code-snippet vuln-line php_extract_overwrite_auth
    if ($is_admin) {
        return BenchmarkResponse::ok('admin dashboard');
    }
    return BenchmarkResponse::error('forbidden', 403);
}
// vuln-code-snippet end php_extract_overwrite_auth
