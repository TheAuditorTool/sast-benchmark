<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_encrypted_cookie_token
function csrf043(BenchmarkRequest $req): BenchmarkResponse {
    $encryptedToken = $req->cookie('csrf_enc');
    $submitted = $req->post('csrf_token');
    $decrypted = decryptToken($encryptedToken); // vuln-code-snippet safe-line php_csrf_encrypted_cookie_token
    if (!hash_equals($decrypted, (string) $submitted)) {
        return BenchmarkResponse::badRequest('CSRF token mismatch');
    }
    performAction($req->post('data'));
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_csrf_encrypted_cookie_token
