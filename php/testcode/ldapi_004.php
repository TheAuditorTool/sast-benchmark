<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_bind_validated
function ldapi004(BenchmarkRequest $req): BenchmarkResponse {
    $user = $req->param('username');
    $pass = $req->post('password');
    if (!preg_match('/^[a-zA-Z0-9_]+$/', $user)) { // vuln-code-snippet safe-line php_ldapi_bind_validated
        return BenchmarkResponse::badRequest('Invalid username format');
    }
    $conn = ldap_connect('ldap://localhost');
    $dn = "uid=" . $user . ",dc=example,dc=com";
    $bound = ldap_bind($conn, $dn, $pass);
    return BenchmarkResponse::json(['authenticated' => $bound]);
}
// vuln-code-snippet end php_ldapi_bind_validated
