<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_basename_disp
function headerinj016(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    $safe = basename($filename);
    header("Content-Disposition: attachment; filename=\"" . $safe . "\""); // vuln-code-snippet safe-line php_headerinj_basename_disp
    return BenchmarkResponse::ok('File download');
}
// vuln-code-snippet end php_headerinj_basename_disp
