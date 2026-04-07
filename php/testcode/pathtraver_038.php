<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_ctype_alnum_filename
function pathtraver038(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('file');
    $noSpecial = str_replace(['-', '_', '.'], '', $filename);
    if (!ctype_alnum($noSpecial)) {
        return BenchmarkResponse::badRequest('invalid filename');
    }
    $content = file_get_contents('/var/app/files/' . $filename); // vuln-code-snippet safe-line php_pt_ctype_alnum_filename
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_ctype_alnum_filename
