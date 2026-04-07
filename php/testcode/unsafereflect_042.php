<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_closure_from_builtin
function unsafereflect042(BenchmarkRequest $req): BenchmarkResponse {
    $fn = Closure::fromCallable('htmlspecialchars'); // vuln-code-snippet safe-line php_reflect_closure_from_builtin
    $result = $fn($req->param('input'));
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_closure_from_builtin
