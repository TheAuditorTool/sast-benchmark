<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_headerinj_intval_userid
function headerinj039(BenchmarkRequest $req): BenchmarkResponse {
    $userId = $req->param('id');
    header('X-User-Id: ' . intval($userId)); // vuln-code-snippet safe-line php_headerinj_intval_userid
    return BenchmarkResponse::ok('user id set');
}
// vuln-code-snippet end php_headerinj_intval_userid
