<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_filename_allowlist
function pathtraver031(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $allowed = ['readme.txt', 'license.txt', 'changelog.txt'];
    if (!in_array($filename, $allowed, true)) {
        return BenchmarkResponse::badRequest('not allowed');
    }
    $content = file_get_contents('/var/app/public/' . $filename); // vuln-code-snippet safe-line php_pt_filename_allowlist
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_filename_allowlist
