<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_link_preload
function headerinj027(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    header('Link: <' . $url . '>; rel=preload'); // vuln-code-snippet vuln-line php_headerinj_link_preload
    return BenchmarkResponse::ok('preload hint set');
}
// vuln-code-snippet end php_headerinj_link_preload
