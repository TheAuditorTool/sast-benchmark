<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_proc_cwd
function cmdi028(BenchmarkRequest $req): BenchmarkResponse {
    $cwd = $req->param('cwd');
    $descriptors = [0 => ['pipe','r'], 1 => ['pipe','w'], 2 => ['pipe','w']];
    $proc = proc_open('ls -la', $descriptors, $pipes, $cwd); // vuln-code-snippet vuln-line php_cmdi_proc_cwd
    $output = stream_get_contents($pipes[1]);
    proc_close($proc);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_cmdi_proc_cwd
