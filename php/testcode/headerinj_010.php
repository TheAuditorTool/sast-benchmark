<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_content_disp
function headerinj010(BenchmarkRequest $req): BenchmarkResponse {
    $filename = $req->param('filename');
    header("Content-Disposition: attachment; filename=\"" . $filename . "\""); // vuln-code-snippet vuln-line php_headerinj_content_disp
    return BenchmarkResponse::ok(file_get_contents('/tmp/exports/report.csv'));
}
// vuln-code-snippet end php_headerinj_content_disp
