<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_phar_hardcoded
function deserial_phar_hardcoded(BenchmarkRequest $req): BenchmarkResponse {
    $content = file_get_contents("phar:///var/app/archive.phar"); // vuln-code-snippet safe-line php_deser_phar_hardcoded
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_deser_phar_hardcoded
