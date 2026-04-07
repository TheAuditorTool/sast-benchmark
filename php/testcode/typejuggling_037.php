<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_is_string_length_hash
function typejuggling037(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $stored = getenv('TOKEN');
    if (!is_string($token) || strlen($token) !== 64 || !hash_equals($stored, $token)) { // vuln-code-snippet safe-line php_tj_is_string_length_hash
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}
// vuln-code-snippet end php_tj_is_string_length_hash
