<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_password_verify
function typejuggling010(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('password');
    $hash = '$2y$10$abcdefghijklmnopqrstuuABCDEFGHIJKLMNOPQRSTUVWXYZ01234';
    if (password_verify($input, $hash)) { // vuln-code-snippet safe-line php_tj_password_verify
        return BenchmarkResponse::ok('authenticated');
    }
    return BenchmarkResponse::error('denied', 403);
}
// vuln-code-snippet end php_tj_password_verify
