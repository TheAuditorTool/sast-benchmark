<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_allowlist_callback
function codeinj034(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['trim', 'strtolower', 'strtoupper', 'strlen'];
    $idx = (int) $req->param('idx');
    $fn = $allowed[$idx] ?? 'trim';
    $result = $fn($req->param('value')); // vuln-code-snippet safe-line php_codeinj_allowlist_callback
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_allowlist_callback
