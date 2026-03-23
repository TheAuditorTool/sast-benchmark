<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_multihop_sanitized
function sanitizeAndRun(string $arg): string {
    $output = [];
    exec("ls " . escapeshellarg($arg), $output); // vuln-code-snippet safe-line php_cmdi_multihop_sanitized
    return implode("\n", $output);
}

function cmdi018(BenchmarkRequest $req): BenchmarkResponse {
    $dir = $req->param('dir');
    $result = sanitizeAndRun($dir);
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_cmdi_multihop_sanitized
