<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_ctype_digit
function typejuggling015(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    if (!ctype_digit($id)) { // vuln-code-snippet safe-line php_tj_ctype_digit
        return BenchmarkResponse::badRequest('ID must be a positive integer');
    }
    return BenchmarkResponse::json(['user_id' => (int)$id]);
}
// vuln-code-snippet end php_tj_ctype_digit
