<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_readfile_allowlist
function pathtraver_readfile_allowlist(BenchmarkRequest $req): BenchmarkResponse {
    $allowedFiles = ['readme' => '/docs/readme.txt', 'license' => '/docs/license.txt', 'changelog' => '/docs/changelog.txt'];
    $fileId = $req->param('file_id');
    if (!isset($allowedFiles[$fileId])) {
        return BenchmarkResponse::badRequest("Unknown file");
    }
    ob_start();
    readfile($allowedFiles[$fileId]); // vuln-code-snippet safe-line php_pt_readfile_allowlist
    $content = ob_get_clean();
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_readfile_allowlist
