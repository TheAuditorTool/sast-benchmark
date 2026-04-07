<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_rememberme_no_rotation
function securecookie026(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('user_id');
    $token = md5($userId . 'salt');
    setcookie('remember', $token, time() + 86400 * 30, '/', '', true, true); // vuln-code-snippet vuln-line php_cookie_rememberme_no_rotation
    return BenchmarkResponse::ok('remember me set');
}
// vuln-code-snippet end php_cookie_rememberme_no_rotation
