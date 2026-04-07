<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_password_verify_safe
function typejuggling034(BenchmarkRequest $req): BenchmarkResponse {
    $pass = $req->param('pass');
    $hash = getenv('STORED_HASH');
    if (!password_verify($pass, $hash)) { // vuln-code-snippet safe-line php_tj_password_verify_safe
        return BenchmarkResponse::badRequest('invalid');
    }
    return BenchmarkResponse::ok('ok');
}
// vuln-code-snippet end php_tj_password_verify_safe
