<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_shell_exec
function cmdi007(BenchmarkRequest $req): BenchmarkResponse {
    $pattern = $req->param('pattern');
    $result = shell_exec("grep " . $pattern . " file.txt"); // vuln-code-snippet vuln-line php_cmdi_shell_exec
    return BenchmarkResponse::ok($result ?? "");
}
// vuln-code-snippet end php_cmdi_shell_exec
