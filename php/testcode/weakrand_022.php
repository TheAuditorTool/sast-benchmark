<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakrand_microtime_uid
function weakrand022(BenchmarkRequest $req): BenchmarkResponse {
    $uid = str_replace('.', '', microtime(true)); // vuln-code-snippet vuln-line php_weakrand_microtime_uid
    return BenchmarkResponse::ok((string)$uid);
}
// vuln-code-snippet end php_weakrand_microtime_uid
