<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_weakhash_md5_salt
function weakhash013(BenchmarkRequest $req): BenchmarkResponse {
    $password = $req->post('password');
    $salt = bin2hex(random_bytes(8));
    $hash = md5($salt . $password); // vuln-code-snippet vuln-line php_weakhash_md5_salt
    return BenchmarkResponse::json(['hash' => $salt . ':' . $hash]);
}
// vuln-code-snippet end php_weakhash_md5_salt
