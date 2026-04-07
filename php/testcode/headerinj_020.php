<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_location_split
function headerinj020(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    header('Location: ' . $url); // vuln-code-snippet vuln-line php_headerinj_location_split
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_headerinj_location_split
