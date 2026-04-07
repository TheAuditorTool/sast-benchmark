<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_date_no_input
function cmdi040(BenchmarkRequest $req): BenchmarkResponse {
    $date = date('Y-m-d');
    $output = [];
    exec("find /var/logs -name 'app-$date.log'", $output); // vuln-code-snippet safe-line php_cmdi_date_no_input
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_date_no_input
