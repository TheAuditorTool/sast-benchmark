<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_base64_json_only
function deserial044(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $decoded = json_decode(base64_decode($token), true); // vuln-code-snippet safe-line php_deser_base64_json_only
    if (!is_array($decoded)) {
        return BenchmarkResponse::badRequest('invalid token');
    }
    return BenchmarkResponse::json($decoded);
}
// vuln-code-snippet end php_deser_base64_json_only
