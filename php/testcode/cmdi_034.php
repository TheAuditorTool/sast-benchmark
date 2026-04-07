<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_intval_arg
function cmdi034(BenchmarkRequest $req): BenchmarkResponse {
    $count = $req->param('count');
    $output = [];
    exec("ping -c " . intval($count) . " 8.8.8.8", $output); // vuln-code-snippet safe-line php_cmdi_intval_arg
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_intval_arg
