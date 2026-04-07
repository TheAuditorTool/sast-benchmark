<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_zero_string_false
function typejuggling025(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->param('val');
    if ($val == false) { // vuln-code-snippet vuln-line php_tj_zero_string_false
        return BenchmarkResponse::ok('empty');
    }
    return BenchmarkResponse::ok('has value');
}
// vuln-code-snippet end php_tj_zero_string_false
