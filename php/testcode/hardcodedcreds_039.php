<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_hardcoded_placeholder_replace
function hardcodedcreds039(BenchmarkRequest $req): BenchmarkResponse {
    define('API_KEY', 'REPLACE_IN_DEPLOY'); // vuln-code-snippet safe-line php_hardcoded_placeholder_replace
    $endpoint = $req->param('endpoint');
    $ch = curl_init($endpoint);
    curl_setopt($ch, CURLOPT_HTTPHEADER, ['X-API-Key: ' . API_KEY]);
    curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
    $result = curl_exec($ch);
    return BenchmarkResponse::ok((string)$result);
}
// vuln-code-snippet end php_hardcoded_placeholder_replace
