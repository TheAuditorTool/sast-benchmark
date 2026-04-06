<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_list_extract_combo
function variablevars012(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('fields');
    $parts = explode(',', $input);
    $a = $parts[0] ?? 'default_a';
    $b = $parts[1] ?? 'default_b';
    $$a = 'overwritten'; // vuln-code-snippet vuln-line php_vv_list_extract_combo
    $$b = 'overwritten';
    return BenchmarkResponse::ok('Done');
}
// vuln-code-snippet end php_vv_list_extract_combo
