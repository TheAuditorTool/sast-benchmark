<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_realpath_check
function pathtraver_realpath_check(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = "/uploads";
    $filename = $req->param('filename');
    $resolved = realpath($baseDir . "/" . $filename);
    if ($resolved === false || !str_starts_with($resolved, $baseDir)) {
        return BenchmarkResponse::badRequest("Invalid path");
    }
    $content = file_get_contents($resolved); // vuln-code-snippet safe-line php_pt_realpath_check
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_realpath_check
