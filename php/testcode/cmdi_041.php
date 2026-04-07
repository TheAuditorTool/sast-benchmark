<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_regex_ip_gate
function cmdi041(BenchmarkRequest $req): BenchmarkResponse {
    $ip = $req->param('ip');
    if (!preg_match('/^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$/', $ip)) {
        return BenchmarkResponse::badRequest('invalid ip');
    }
    $output = [];
    exec("ping -c 1 $ip", $output); // vuln-code-snippet safe-line php_cmdi_regex_ip_gate
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_regex_ip_gate
