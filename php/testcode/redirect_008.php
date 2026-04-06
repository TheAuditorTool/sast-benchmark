<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_redirect_data_uri
function redirect008(BenchmarkRequest $req): BenchmarkResponse {
    $target = $req->param('goto');
    return BenchmarkResponse::redirect($target); // vuln-code-snippet vuln-line php_redirect_data_uri
}
// vuln-code-snippet end php_redirect_data_uri
