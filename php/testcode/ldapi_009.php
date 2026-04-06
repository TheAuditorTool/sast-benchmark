<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_compare_tainted_dn
function ldapi009(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $dn = $req->param('dn');
    $result = ldap_compare($conn, $dn, 'memberOf', 'cn=admins,dc=example,dc=com'); // vuln-code-snippet vuln-line php_ldapi_compare_tainted_dn
    return BenchmarkResponse::json(['is_admin' => $result]);
}
// vuln-code-snippet end php_ldapi_compare_tainted_dn
