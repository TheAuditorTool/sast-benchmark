<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_mkdir_sanitized
function pathtraver_mkdir_sanitized(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('project');
    $safeName = preg_replace('/[^a-zA-Z0-9_-]/', '', $name);
    mkdir("/projects/" . $safeName, 0755, true); // vuln-code-snippet safe-line php_pt_mkdir_sanitized
    return BenchmarkResponse::ok("Project directory created");
}
// vuln-code-snippet end php_pt_mkdir_sanitized
