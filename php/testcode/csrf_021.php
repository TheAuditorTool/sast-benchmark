<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_samesite_lax_get
function csrf021(BenchmarkRequest $req): BenchmarkResponse {
    setcookie('session', getSessionId(), [
        'samesite' => 'Lax',
        'httponly' => true,
    ]);
    $userId = (int) $req->param('user');
    $newEmail = $req->param('email');
    updateEmail($userId, $newEmail); // vuln-code-snippet vuln-line php_csrf_samesite_lax_get
    return BenchmarkResponse::ok('email updated');
}
// vuln-code-snippet end php_csrf_samesite_lax_get
