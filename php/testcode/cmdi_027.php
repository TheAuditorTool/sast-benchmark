<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_popen_tar
function cmdi027(BenchmarkRequest $req): BenchmarkResponse {
    $userDir = $req->param('dir');
    $handle = popen("tar czf backup.tar $userDir", 'r'); // vuln-code-snippet vuln-line php_cmdi_popen_tar
    $output = fread($handle, 4096);
    pclose($handle);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_cmdi_popen_tar
