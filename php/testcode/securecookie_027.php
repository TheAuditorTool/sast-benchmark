<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cookie_value_from_input
function securecookie027(BenchmarkRequest $req): BenchmarkResponse {
    $pref = $req->param('theme');
    setcookie('pref', $pref, time() + 86400, '/', '', false, false); // vuln-code-snippet vuln-line php_cookie_value_from_input
    return BenchmarkResponse::ok('pref saved');
}
// vuln-code-snippet end php_cookie_value_from_input
