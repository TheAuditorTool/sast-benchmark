<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_from_callable
function codeinj017(BenchmarkRequest $req): BenchmarkResponse {
    $action = $req->param('action');
    $allowed = ['strlen', 'strtoupper', 'strtolower', 'trim'];
    if (!in_array($action, $allowed, true)) {
        return BenchmarkResponse::badRequest('Function not allowed');
    }
    $fn = Closure::fromCallable($action); // vuln-code-snippet safe-line php_codeinj_from_callable
    $result = $fn($req->param('input'));
    return BenchmarkResponse::ok((string)$result);
}
// vuln-code-snippet end php_codeinj_from_callable
