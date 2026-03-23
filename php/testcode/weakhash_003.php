<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_sha1_token
function weakhash003(BenchmarkRequest $req): BenchmarkResponse {
    $token = $req->param('token');
    $hashed = sha1($token); // vuln-code-snippet vuln-line php_weakhash_sha1_token
    return BenchmarkResponse::json(['token_hash' => $hashed]);
}
// vuln-code-snippet end php_weakhash_sha1_token
