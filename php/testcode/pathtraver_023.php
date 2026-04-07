<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_opendir_enum
function pathtraver023(BenchmarkRequest $req): BenchmarkResponse {
    $userDir = $req->param('dir');
    $dh = opendir($userDir); // vuln-code-snippet vuln-line php_pt_opendir_enum
    $files = [];
    while (($file = readdir($dh)) !== false) {
        $files[] = $file;
    }
    closedir($dh);
    return BenchmarkResponse::ok(implode("\n", $files));
}
// vuln-code-snippet end php_pt_opendir_enum
