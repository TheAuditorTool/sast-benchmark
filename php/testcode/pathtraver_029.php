<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_realpath_strpos
function pathtraver029(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = realpath('/var/app/files');
    $input = $req->param('file');
    $real = realpath($baseDir . '/' . $input);
    if ($real === false || strpos($real, $baseDir) !== 0) {
        return BenchmarkResponse::badRequest('access denied');
    }
    $content = file_get_contents($real); // vuln-code-snippet safe-line php_pt_realpath_strpos
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_realpath_strpos
