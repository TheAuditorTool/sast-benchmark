<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_meta_refresh
function redirect003(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $html = '<meta http-equiv="refresh" content="0;url=' . $url . '">'; // vuln-code-snippet vuln-line php_redirect_meta_refresh
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_redirect_meta_refresh
