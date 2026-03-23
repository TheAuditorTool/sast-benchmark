<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_switch_loose
function typejuggling003(BenchmarkRequest $req): BenchmarkResponse {
    $role = $req->param('role');
    switch ($role) { // vuln-code-snippet vuln-line php_tj_switch_loose
        case 0:
            return BenchmarkResponse::ok('admin access granted');
        case 'editor':
            return BenchmarkResponse::ok('editor access');
        default:
            return BenchmarkResponse::ok('guest access');
    }
}
// vuln-code-snippet end php_tj_switch_loose
