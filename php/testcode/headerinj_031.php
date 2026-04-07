<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_arbitrary_loop
function headerinj031(BenchmarkRequest $req): BenchmarkResponse {
    $headers = json_decode($req->bodyStr(), true)['headers'] ?? [];
    foreach ($headers as $h) {
        header($h); // vuln-code-snippet vuln-line php_headerinj_arbitrary_loop
    }
    return BenchmarkResponse::ok('headers set');
}
// vuln-code-snippet end php_headerinj_arbitrary_loop
