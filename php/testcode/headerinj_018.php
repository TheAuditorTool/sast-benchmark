<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_int_cast
function headerinj018(BenchmarkRequest $req): BenchmarkResponse {
    $count = $req->param('count');
    header("X-Result-Count: " . (int)$count); // vuln-code-snippet safe-line php_headerinj_int_cast
    return BenchmarkResponse::ok('Results returned');
}
// vuln-code-snippet end php_headerinj_int_cast
