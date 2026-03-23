<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_backtick
function cmdi009(BenchmarkRequest $req): BenchmarkResponse {
    $domain = $req->param('domain');
    $result = `nslookup $domain`; // vuln-code-snippet vuln-line php_cmdi_backtick
    return BenchmarkResponse::ok($result ?? "");
}
// vuln-code-snippet end php_cmdi_backtick
