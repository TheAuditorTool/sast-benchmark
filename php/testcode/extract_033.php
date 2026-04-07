<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_import_request_vars
function extract033(BenchmarkRequest $req): BenchmarkResponse {
    // Legacy PHP 5.x pattern
    import_request_variables('gpc', ''); // vuln-code-snippet vuln-line php_extract_import_request_vars
    $lang = $lang ?? 'en';
    return BenchmarkResponse::ok("lang=$lang");
}
// vuln-code-snippet end php_extract_import_request_vars
