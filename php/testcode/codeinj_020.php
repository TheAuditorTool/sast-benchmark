<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_eval_base64
function codeinj020(BenchmarkRequest $req): BenchmarkResponse {
    $encoded = $req->post('code');
    $decoded = base64_decode($encoded);
    eval($decoded); // vuln-code-snippet vuln-line php_codeinj_eval_base64
    return BenchmarkResponse::ok('executed');
}
// vuln-code-snippet end php_codeinj_eval_base64
