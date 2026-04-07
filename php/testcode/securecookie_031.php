<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_rememberme_predictable
function securecookie031(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('user_id');
    $token = $userId . ':' . date('YmdHis'); // vuln-code-snippet vuln-line php_cookie_rememberme_predictable
    setcookie('remember', $token, time() + 86400 * 30, '/', '', true, true);
    return BenchmarkResponse::ok('remember me set');
}
// vuln-code-snippet end php_cookie_rememberme_predictable
