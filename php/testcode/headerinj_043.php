<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_ctype_alnum_realm
function headerinj043(BenchmarkRequest $req): BenchmarkResponse {
    $realm = $req->param('realm');
    if (!ctype_alnum($realm)) {
        return BenchmarkResponse::badRequest('invalid realm');
    }
    header('WWW-Authenticate: Basic realm="' . $realm . '"'); // vuln-code-snippet safe-line php_headerinj_ctype_alnum_realm
    return BenchmarkResponse::error('Unauthorized', 401);
}
// vuln-code-snippet end php_headerinj_ctype_alnum_realm
