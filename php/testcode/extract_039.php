<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_compact_literals
function extract039(BenchmarkRequest $req): BenchmarkResponse {
    $name  = htmlspecialchars($req->param('name'));
    $email = htmlspecialchars($req->param('email'));
    $out = compact('name', 'email'); // vuln-code-snippet safe-line php_extract_compact_literals
    return BenchmarkResponse::json($out);
}
// vuln-code-snippet end php_extract_compact_literals
