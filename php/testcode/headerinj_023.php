<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_www_auth_realm
function headerinj023(BenchmarkRequest $req): BenchmarkResponse {
    $realm = $req->param('realm');
    header('WWW-Authenticate: Basic realm="' . $realm . '"'); // vuln-code-snippet vuln-line php_headerinj_www_auth_realm
    return BenchmarkResponse::error('Unauthorized', 401);
}
// vuln-code-snippet end php_headerinj_www_auth_realm
