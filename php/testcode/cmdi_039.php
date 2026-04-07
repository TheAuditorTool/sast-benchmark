<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_proc_array_descriptor
function cmdi039(BenchmarkRequest $req): BenchmarkResponse {
    $safePath = '/var/app/data';
    $descriptors = [0 => ['pipe','r'], 1 => ['pipe','w']];
    $proc = proc_open(['ls', '-la', $safePath], $descriptors, $pipes); // vuln-code-snippet safe-line php_cmdi_proc_array_descriptor
    $output = stream_get_contents($pipes[1]);
    proc_close($proc);
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_cmdi_proc_array_descriptor
