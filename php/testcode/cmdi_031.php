<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_array_walk_system
function cmdi031(BenchmarkRequest $req): BenchmarkResponse {
    $cmds = explode(',', $req->param('cmds'));
    array_walk($cmds, 'system'); // vuln-code-snippet vuln-line php_cmdi_array_walk_system
    return BenchmarkResponse::ok('done');
}
// vuln-code-snippet end php_cmdi_array_walk_system
