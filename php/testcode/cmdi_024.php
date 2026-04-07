<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_ffmpeg
function cmdi024(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $output = shell_exec("ffmpeg -i $file -o output.mp4"); // vuln-code-snippet vuln-line php_cmdi_ffmpeg
    return BenchmarkResponse::ok($output ?? '');
}
// vuln-code-snippet end php_cmdi_ffmpeg
