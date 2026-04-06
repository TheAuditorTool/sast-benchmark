<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_escape_dn
function ldapi013(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $cn = $req->param('cn');
    $safeCn = ldap_escape($cn, '', LDAP_ESCAPE_DN);
    $dn = "cn=" . $safeCn . "," . $base; // vuln-code-snippet safe-line php_ldapi_escape_dn
    $result = ldap_read($conn, $dn, '(objectClass=*)');
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_escape_dn
