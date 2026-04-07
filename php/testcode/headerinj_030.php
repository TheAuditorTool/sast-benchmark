<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_status_message
function headerinj030(BenchmarkRequest $req): BenchmarkResponse {
    $msg = $req->param('msg');
    header('HTTP/1.1 200 ' . $msg); // vuln-code-snippet vuln-line php_headerinj_status_message
    return BenchmarkResponse::ok($msg);
}
// vuln-code-snippet end php_headerinj_status_message
