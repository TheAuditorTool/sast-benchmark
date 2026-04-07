<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_pathinfo_ext_prefix
function pathtraver033(BenchmarkRequest $req): BenchmarkResponse {
    $f = $req->param('file');
    $ext = pathinfo($f, PATHINFO_EXTENSION);
    $allowed_ext = ['jpg', 'png', 'gif'];
    if (!in_array(strtolower($ext), $allowed_ext, true)) {
        return BenchmarkResponse::badRequest('invalid extension');
    }
    $content = file_get_contents('/var/app/uploads/' . basename($f)); // vuln-code-snippet safe-line php_pt_pathinfo_ext_prefix
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_pathinfo_ext_prefix
