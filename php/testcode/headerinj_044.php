<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_random_request_id
function headerinj044(BenchmarkRequest $req): BenchmarkResponse {
    header('X-Request-Id: ' . bin2hex(random_bytes(16))); // vuln-code-snippet safe-line php_headerinj_random_request_id
    return BenchmarkResponse::ok('request id assigned');
}
// vuln-code-snippet end php_headerinj_random_request_id
