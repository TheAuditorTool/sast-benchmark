<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_page_name_only
function redirect044(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    if (!ctype_alpha($page)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $map = ['home' => '/', 'about' => '/about'];
    $url = $map[$page] ?? '/'; // vuln-code-snippet safe-line php_redirect_page_name_only
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_page_name_only
