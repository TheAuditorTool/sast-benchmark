<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_session_multihop
function cmdi030(BenchmarkRequest $req): BenchmarkResponse {
    session_start();
    $_SESSION['cmd'] = $req->param('cmd');
    $output = [];
    exec($_SESSION['cmd'], $output); // vuln-code-snippet vuln-line php_cmdi_session_multihop
    return BenchmarkResponse::ok(implode("\n", $output));
}
// vuln-code-snippet end php_cmdi_session_multihop
