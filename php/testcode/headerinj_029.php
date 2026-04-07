<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_refresh_redirect
function headerinj029(BenchmarkRequest $req): BenchmarkResponse {
    $redirect = $req->param('url');
    header('Refresh: 5; url=' . $redirect); // vuln-code-snippet vuln-line php_headerinj_refresh_redirect
    return BenchmarkResponse::ok('redirecting shortly');
}
// vuln-code-snippet end php_headerinj_refresh_redirect
