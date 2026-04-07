<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_js_location
function redirect018(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    echo "<script>location.href='$url'</script>"; // vuln-code-snippet vuln-line php_redirect_js_location
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_redirect_js_location
