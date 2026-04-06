<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_uniqid
function weakrand007(BenchmarkRequest $req): BenchmarkResponse {
    $token = uniqid('sess_', true); // vuln-code-snippet vuln-line php_weakrand_uniqid
    return BenchmarkResponse::json(['session_token' => $token]);
}
// vuln-code-snippet end php_weakrand_uniqid
