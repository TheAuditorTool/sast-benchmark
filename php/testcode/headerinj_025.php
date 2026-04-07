<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_via_proxy
function headerinj025(BenchmarkRequest $req): BenchmarkResponse {
    $via = $req->header('Via');
    header('Via: ' . $via); // vuln-code-snippet vuln-line php_headerinj_via_proxy
    return BenchmarkResponse::ok('proxied');
}
// vuln-code-snippet end php_headerinj_via_proxy
