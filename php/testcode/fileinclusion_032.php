<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_hardcoded_include
function fileinclusion032(BenchmarkRequest $req): BenchmarkResponse {
    include __DIR__ . '/templates/header.php'; // vuln-code-snippet safe-line php_fi_hardcoded_include
    return BenchmarkResponse::ok('Header rendered');
}
// vuln-code-snippet end php_fi_hardcoded_include
