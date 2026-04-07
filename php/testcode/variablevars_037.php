<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_closure_scope_isolated
function variablevars037(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('input');
    $result = (function() use ($data) { // vuln-code-snippet safe-line php_vv_closure_scope_isolated
        $local = $data;
        return $local;
    })();
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_vv_closure_scope_isolated
