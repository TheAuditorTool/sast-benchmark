<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_x_redirect_by
function headerinj033(BenchmarkRequest $req): BenchmarkResponse {
    $appName = $req->param('app');
    header('X-Redirect-By: ' . $appName); // vuln-code-snippet vuln-line php_headerinj_x_redirect_by
    return BenchmarkResponse::redirect('https://example.com/');
}
// vuln-code-snippet end php_headerinj_x_redirect_by
