<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_realpath_validated
function fileinclusion008(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = realpath(__DIR__ . '/templates');
    $file = $req->param('template');
    $resolved = realpath($baseDir . '/' . $file);
    if ($resolved === false || !str_starts_with($resolved, $baseDir)) {
        return BenchmarkResponse::badRequest("invalid template path");
    }
    include($resolved); // vuln-code-snippet safe-line php_fi_realpath_validated
    return BenchmarkResponse::ok("template loaded");
}
// vuln-code-snippet end php_fi_realpath_validated
