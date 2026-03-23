<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_md5_password
function weakhash001(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $hashed = md5($password); // vuln-code-snippet vuln-line php_weakhash_md5_password
    return BenchmarkResponse::json(['hash' => $hashed]);
}
// vuln-code-snippet end php_weakhash_md5_password
