<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_filter_validate_ip
function cmdi036(BenchmarkRequest $req): BenchmarkResponse {
    $ip = $req->param('ip');
    if (!filter_var($ip, FILTER_VALIDATE_IP)) {
        return BenchmarkResponse::badRequest('invalid ip');
    }
    $output = [];
    exec("ping -c 3 " . $ip, $output); // vuln-code-snippet safe-line php_cmdi_filter_validate_ip
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_filter_validate_ip
