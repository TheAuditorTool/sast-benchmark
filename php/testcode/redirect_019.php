<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_meta_refresh_inject
function redirect019(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    echo "<meta http-equiv='refresh' content='0;url=$url'>"; // vuln-code-snippet vuln-line php_redirect_meta_refresh_inject
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_meta_refresh_inject
