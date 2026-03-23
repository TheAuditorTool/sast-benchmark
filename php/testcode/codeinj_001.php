<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_eval
function codeinj_eval(BenchmarkRequest $req): BenchmarkResponse {
    $code = $req->post('code');
    $result = eval($code); // vuln-code-snippet vuln-line php_codeinj_eval
    return BenchmarkResponse::ok((string) $result);
}
// vuln-code-snippet end php_codeinj_eval
