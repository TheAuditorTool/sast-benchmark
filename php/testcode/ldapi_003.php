<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_bind_tainted
function ldapi003(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('username');
    $pass = $req->post('password');
    $conn = ldap_connect('ldap://localhost');
    $dn = "uid=" . $input . ",dc=example,dc=com";
    $bound = ldap_bind($conn, $dn, $pass); // vuln-code-snippet vuln-line php_ldapi_bind_tainted
    return BenchmarkResponse::json(['authenticated' => $bound]);
}
// vuln-code-snippet end php_ldapi_bind_tainted
