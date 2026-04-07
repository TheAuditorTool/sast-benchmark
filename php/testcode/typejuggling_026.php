<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_array_search_non_strict
function typejuggling026(BenchmarkRequest $req): BenchmarkResponse {
    $needle = $req->param('val');
    $arr = ['admin', 'user', 'guest'];
    $pos = array_search($needle, $arr); // vuln-code-snippet vuln-line php_tj_array_search_non_strict // Legacy PHP 7.x pattern
    if ($pos !== false) {
        return BenchmarkResponse::ok('found:' . $pos);
    }
    return BenchmarkResponse::badRequest('not found');
}
// vuln-code-snippet end php_tj_array_search_non_strict
