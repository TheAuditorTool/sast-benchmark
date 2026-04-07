<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_is_numeric_hex_legacy
function typejuggling031(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->param('val');
    if (is_numeric($val)) { // vuln-code-snippet vuln-line php_tj_is_numeric_hex_legacy // Legacy PHP 5.x pattern
        $num = intval($val, 0);
        return BenchmarkResponse::ok((string)$num);
    }
    return BenchmarkResponse::badRequest('not numeric');
}
// vuln-code-snippet end php_tj_is_numeric_hex_legacy
