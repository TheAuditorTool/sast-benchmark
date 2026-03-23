<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_wrapper_blocked
function fileinclusion014(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('path');
    $blockedSchemes = ['data', 'php', 'expect', 'http', 'https', 'ftp'];
    $scheme = parse_url($path, PHP_URL_SCHEME);
    if ($scheme !== null && in_array(strtolower($scheme), $blockedSchemes, true)) {
        return BenchmarkResponse::badRequest("blocked URL scheme: " . $scheme);
    }
    $safe = basename($path);
    include("templates/" . $safe); // vuln-code-snippet safe-line php_fi_wrapper_blocked
    return BenchmarkResponse::ok("content loaded");
}
// vuln-code-snippet end php_fi_wrapper_blocked
