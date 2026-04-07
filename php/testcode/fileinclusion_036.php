<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_intval_file_number
function fileinclusion036(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int) $req->param('id');
    require_once __DIR__ . '/views/' . $id . '.php'; // vuln-code-snippet safe-line php_fi_intval_file_number
    return BenchmarkResponse::ok('Rendered');
}
// vuln-code-snippet end php_fi_intval_file_number
