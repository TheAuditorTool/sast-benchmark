<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_hardcoded_source
function deserial_hardcoded_source(BenchmarkRequest $req): BenchmarkResponse {
    $raw = file_get_contents('/var/cache/app.dat');
    $config = unserialize($raw); // vuln-code-snippet safe-line php_deser_hardcoded_source
    return BenchmarkResponse::json(['cache_ttl' => $config['ttl'] ?? 3600]);
}
// vuln-code-snippet end php_deser_hardcoded_source
