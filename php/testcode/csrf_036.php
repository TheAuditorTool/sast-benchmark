<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_mtls_auth
function csrf036(BenchmarkRequest $req): BenchmarkResponse {
    $clientCert = $_SERVER['SSL_CLIENT_CERT'] ?? '';
    $verified = $_SERVER['SSL_CLIENT_VERIFY'] ?? 'NONE';
    if ($verified !== 'SUCCESS' || empty($clientCert)) { // vuln-code-snippet safe-line php_csrf_mtls_auth
        return BenchmarkResponse::error('client certificate required');
    }
    $subject = $_SERVER['SSL_CLIENT_S_DN'] ?? '';
    return BenchmarkResponse::json(['subject' => $subject, 'action' => 'performed']);
}
// vuln-code-snippet end php_csrf_mtls_auth
