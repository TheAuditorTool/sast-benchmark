<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_location
function headerinj001(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('url');
    header("Location: " . $input); // vuln-code-snippet vuln-line php_headerinj_location
    return BenchmarkResponse::ok('Redirecting...');
}
// vuln-code-snippet end php_headerinj_location
