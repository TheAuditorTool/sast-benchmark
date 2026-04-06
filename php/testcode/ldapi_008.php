<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_modify_injection
function ldapi008(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = $req->param('uid');
    $mail = $req->post('mail');
    ldap_modify($conn, "uid=" . $uid . "," . $base, ['mail' => $mail]); // vuln-code-snippet vuln-line php_ldapi_modify_injection
    return BenchmarkResponse::ok('Profile updated');
}
// vuln-code-snippet end php_ldapi_modify_injection
