<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_session_no_ldap_query
function ldapi047(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $userId = $_SESSION['user_id'] ?? null; // vuln-code-snippet safe-line php_ldapi_session_no_ldap_query
    if (!$userId) {
        return BenchmarkResponse::error('not authenticated', 401);
    }
    return BenchmarkResponse::json(['user_id' => $userId]);
}
// vuln-code-snippet end php_ldapi_session_no_ldap_query
