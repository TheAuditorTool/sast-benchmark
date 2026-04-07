<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_compact_dynamic_chain
function variablevars028(BenchmarkRequest $req): BenchmarkResponse {
    $varName = $req->param('vars');
    $$varName = 'injected';
    $data = compact($$varName); // vuln-code-snippet vuln-line php_vv_compact_dynamic_chain
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_vv_compact_dynamic_chain
