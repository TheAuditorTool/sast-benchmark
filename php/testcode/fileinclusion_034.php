<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_in_array_allowlist
function fileinclusion034(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = $req->param('tpl');
    if (!in_array($tpl, ['home', 'about', 'contact'], true)) { // vuln-code-snippet safe-line php_fi_in_array_allowlist
        return BenchmarkResponse::badRequest('Invalid template');
    }
    include __DIR__ . "/pages/{$tpl}.php";
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_in_array_allowlist
