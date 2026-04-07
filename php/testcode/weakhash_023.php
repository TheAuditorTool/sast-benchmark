<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_md5_csrf_token
function weakhash023(BenchmarkRequest $req): BenchmarkResponse {
    $token = md5(session_id() . time()); // vuln-code-snippet vuln-line php_weakhash_md5_csrf_token
    setcookie('csrf', $token);
    return BenchmarkResponse::ok('csrf set');
}
// vuln-code-snippet end php_weakhash_md5_csrf_token
