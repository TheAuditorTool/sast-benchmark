<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_proc_open
function cmdi011(BenchmarkRequest $req): BenchmarkResponse {
    $cmd = $req->param('cmd');
    $descriptors = [1 => ['pipe', 'w'], 2 => ['pipe', 'w']];
    $proc = proc_open($cmd, $descriptors, $pipes); // vuln-code-snippet vuln-line php_cmdi_proc_open
    $output = stream_get_contents($pipes[1]);
    fclose($pipes[1]);
    fclose($pipes[2]);
    proc_close($proc);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_cmdi_proc_open
