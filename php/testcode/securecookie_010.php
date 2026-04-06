<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_short_expiry_no_flags
function securecookie010(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->post('preference');
    setcookie('tmp_pref', $val, time() + 60); // vuln-code-snippet vuln-line php_cookie_short_expiry_no_flags
    return BenchmarkResponse::ok('Temporary preference saved');
}
// vuln-code-snippet end php_cookie_short_expiry_no_flags
