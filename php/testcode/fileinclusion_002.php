<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_include_allowlist
function fileinclusion002(BenchmarkRequest $req): BenchmarkResponse {
    $allowedPages = ['home' => 'pages/home.php', 'about' => 'pages/about.php', 'contact' => 'pages/contact.php'];
    $page = $req->param('page');
    if (!isset($allowedPages[$page])) {
        return BenchmarkResponse::badRequest("invalid page");
    }
    include($allowedPages[$page]); // vuln-code-snippet safe-line php_fi_include_allowlist
    return BenchmarkResponse::ok("page loaded");
}
// vuln-code-snippet end php_fi_include_allowlist
