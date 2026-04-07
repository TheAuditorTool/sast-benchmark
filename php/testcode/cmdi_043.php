<?php
require_once __DIR__ . '/shared.php';

define('REPORT_CMD', '/usr/local/bin/generate-report');

// vuln-code-snippet start php_cmdi_config_constant_cmd
function cmdi043(BenchmarkRequest $req): BenchmarkResponse {
    $output = [];
    exec(REPORT_CMD . ' --format pdf', $output); // vuln-code-snippet safe-line php_cmdi_config_constant_cmd
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_config_constant_cmd
