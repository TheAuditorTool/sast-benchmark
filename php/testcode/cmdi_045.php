<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_config_flag_only
function cmdi045(BenchmarkRequest $req): BenchmarkResponse {
    $verbose = (bool)getenv('DEBUG_VERBOSE');
    $flag = $verbose ? '-v' : '';
    $output = [];
    exec("/usr/local/bin/check-service $flag", $output); // vuln-code-snippet safe-line php_cmdi_config_flag_only
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_config_flag_only
