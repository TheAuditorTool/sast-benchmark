<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_switch_coerce
function typejuggling021(BenchmarkRequest $req): BenchmarkResponse {
    $type = $req->param('type');
    switch ($type) { // vuln-code-snippet vuln-line php_tj_switch_coerce // Legacy PHP 7.x pattern
        case 1:
            return BenchmarkResponse::ok('admin');
        case 2:
            return BenchmarkResponse::ok('user');
    }
    return BenchmarkResponse::badRequest('denied');
}
// vuln-code-snippet end php_tj_switch_coerce
