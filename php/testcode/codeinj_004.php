<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_assert_bool
function codeinj_assert_bool(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    assert(is_numeric($id)); // vuln-code-snippet safe-line php_codeinj_assert_bool
    return BenchmarkResponse::ok("ID is valid: " . $id);
}
// vuln-code-snippet end php_codeinj_assert_bool
