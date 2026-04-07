<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_closure_from_callable
function codeinj047(BenchmarkRequest $req): BenchmarkResponse {
    $trimmer = Closure::fromCallable('trim'); // vuln-code-snippet safe-line php_codeinj_closure_from_callable
    $result = $trimmer($req->param('value'));
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_codeinj_closure_from_callable
