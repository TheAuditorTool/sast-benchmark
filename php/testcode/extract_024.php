<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_refs_overwrite
function extract024(BenchmarkRequest $req): BenchmarkResponse {
    $isAdmin = false;
    $role = 'user';
    extract($_POST, EXTR_REFS); // vuln-code-snippet vuln-line php_extract_refs_overwrite
    return BenchmarkResponse::ok("role=$role admin=" . ($isAdmin ? '1' : '0'));
}
// vuln-code-snippet end php_extract_refs_overwrite
