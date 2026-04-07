<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_transfer_encoding
function headerinj024(BenchmarkRequest $req): BenchmarkResponse {
    $enc = $req->param('enc');
    header('Transfer-Encoding: ' . $enc); // vuln-code-snippet vuln-line php_headerinj_transfer_encoding
    return BenchmarkResponse::ok('encoding set');
}
// vuln-code-snippet end php_headerinj_transfer_encoding
