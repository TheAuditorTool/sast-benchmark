<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_response_split
function headerinj012(BenchmarkRequest $req): BenchmarkResponse {
    $dest = $req->param('dest');
    header("Location: " . $dest); // vuln-code-snippet vuln-line php_headerinj_response_split
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_headerinj_response_split
