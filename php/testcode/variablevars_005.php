<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_compact_dynamic
function variablevars005(BenchmarkRequest $req): BenchmarkResponse {
    $varname = $req->param('field');
    $$varname = $req->param('value');
    $data = compact($varname); // vuln-code-snippet vuln-line php_vv_compact_dynamic
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_vv_compact_dynamic
