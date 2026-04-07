<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_putenv_path
function cmdi023(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('path');
    putenv("PATH=$input"); // vuln-code-snippet vuln-line php_cmdi_putenv_path
    $output = [];
    exec('ls', $output);
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_putenv_path
