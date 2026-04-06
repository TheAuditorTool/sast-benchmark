<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_md5_csrf
function weakhash010(BenchmarkRequest $req): BenchmarkResponse {
    $sessionId = session_id();
    $token = md5($sessionId . time()); // vuln-code-snippet vuln-line php_weakhash_md5_csrf
    return BenchmarkResponse::json(['csrf_token' => $token]);
}
// vuln-code-snippet end php_weakhash_md5_csrf
