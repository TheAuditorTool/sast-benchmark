<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_strict_filename_regex
function pathtraver039(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    if (!preg_match('/^[a-zA-Z0-9_-]+\.(jpg|png|pdf)$/', $filename)) {
        return BenchmarkResponse::badRequest('invalid filename');
    }
    $content = file_get_contents('/var/app/files/' . $filename); // vuln-code-snippet safe-line php_pt_strict_filename_regex
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_strict_filename_regex
