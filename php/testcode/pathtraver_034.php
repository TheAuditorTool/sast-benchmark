<?php
require_once __DIR__ . '/shared.php';

define('ASSETS_DIR', '/var/app/assets/');

// vuln-code-snippet start php_pt_config_dir_constant
function pathtraver034(BenchmarkRequest $req): BenchmarkResponse {
    $filename = basename($req->param('file'));
    $content = file_get_contents(ASSETS_DIR . $filename); // vuln-code-snippet safe-line php_pt_config_dir_constant
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_config_dir_constant
