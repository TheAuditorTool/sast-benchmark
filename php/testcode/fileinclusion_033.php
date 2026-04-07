<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_switch_allowlist
function fileinclusion033(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    switch ($page) {
        case 'home':  include __DIR__ . '/pages/home.php';  break; // vuln-code-snippet safe-line php_fi_switch_allowlist
        case 'about': include __DIR__ . '/pages/about.php'; break;
        default:      return BenchmarkResponse::badRequest('Unknown page');
    }
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_switch_allowlist
