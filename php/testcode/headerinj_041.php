<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_gmdate_no_input
function headerinj041(BenchmarkRequest $req): BenchmarkResponse {
    header('Date: ' . gmdate('D, d M Y H:i:s T')); // vuln-code-snippet safe-line php_headerinj_gmdate_no_input
    return BenchmarkResponse::ok('date header set');
}
// vuln-code-snippet end php_headerinj_gmdate_no_input
