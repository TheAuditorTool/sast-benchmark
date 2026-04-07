<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_readonly_get
function csrf031(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('user_id');
    $profile = fetchUserProfile($userId); // vuln-code-snippet safe-line php_csrf_readonly_get
    return BenchmarkResponse::json($profile);
}
// vuln-code-snippet end php_csrf_readonly_get
