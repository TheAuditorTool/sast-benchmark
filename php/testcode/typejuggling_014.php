<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_hash_equals
function typejuggling014(BenchmarkRequest $req): BenchmarkResponse {
    $submitted = $req->post('token');
    $stored = $_SESSION['auth_token'] ?? '';
    if (!hash_equals($stored, $submitted)) { // vuln-code-snippet safe-line php_tj_hash_equals
        return BenchmarkResponse::badRequest('Invalid token');
    }
    return BenchmarkResponse::ok('Authenticated');
}
// vuln-code-snippet end php_tj_hash_equals
