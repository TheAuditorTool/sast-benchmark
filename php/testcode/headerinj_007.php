<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_setcookie_value
function headerinj007(BenchmarkRequest $req): BenchmarkResponse {
    $pref = $req->param('preference');
    setcookie('user_pref', $pref, time() + 3600); // vuln-code-snippet vuln-line php_headerinj_setcookie_value
    return BenchmarkResponse::ok('Preference saved');
}
// vuln-code-snippet end php_headerinj_setcookie_value
