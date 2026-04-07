<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_vv_config_file_key_only
function variablevars038(BenchmarkRequest $req): BenchmarkResponse {
    $configKeys = parse_ini_file('/etc/app/display.ini');
    foreach ($configKeys as $k => $v) {
        $$k = $v; // vuln-code-snippet safe-line php_vv_config_file_key_only
    }
    return BenchmarkResponse::ok('loaded');
}
// vuln-code-snippet end php_vv_config_file_key_only
