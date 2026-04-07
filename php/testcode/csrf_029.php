<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_iframe_post
function csrf029(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->post('action');
    $target = $req->post('target');
    performAdminAction($action, $target); // vuln-code-snippet vuln-line php_csrf_iframe_post
    return BenchmarkResponse::ok('action performed');
}
// vuln-code-snippet end php_csrf_iframe_post
