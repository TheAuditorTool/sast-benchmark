<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_pcntl_exec
function cmdi019(BenchmarkRequest $req): BenchmarkResponse {
    $path = $req->param('executable');
    $args = explode(" ", $req->param('args'));
    pcntl_exec($path, $args); // vuln-code-snippet vuln-line php_cmdi_pcntl_exec
    return BenchmarkResponse::error("exec failed");
}
// vuln-code-snippet end php_cmdi_pcntl_exec
