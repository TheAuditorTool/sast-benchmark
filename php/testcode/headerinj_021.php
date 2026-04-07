<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_setcookie_session
function headerinj021(BenchmarkRequest $req): BenchmarkResponse {
    $sid = $req->param('sid');
    header('Set-Cookie: session=' . $sid . '; Path=/'); // vuln-code-snippet vuln-line php_headerinj_setcookie_session
    return BenchmarkResponse::ok('session set');
}
// vuln-code-snippet end php_headerinj_setcookie_session
