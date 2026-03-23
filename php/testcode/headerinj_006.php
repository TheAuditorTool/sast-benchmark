<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_cookie_safe
function headerinj006(BenchmarkRequest $req): BenchmarkResponse {
    $allowedNames = ['theme', 'lang', 'timezone'];
    $name = $req->param('name');
    $value = $req->param('value');
    if (in_array($name, $allowedNames, true)) { // vuln-code-snippet safe-line php_headerinj_cookie_safe
        setcookie($name, urlencode($value));
        return BenchmarkResponse::ok('Cookie set');
    }
    return BenchmarkResponse::badRequest('Invalid cookie name');
}
// vuln-code-snippet end php_headerinj_cookie_safe
