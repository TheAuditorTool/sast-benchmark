<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_readonly_account
function ldapi039(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    ldap_bind($conn, 'cn=readonly,dc=example,dc=com', getenv('LDAP_READONLY_PASS'));
    $result = ldap_search($conn, $base, '(objectClass=inetOrgPerson)'); // vuln-code-snippet safe-line php_ldapi_readonly_account
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_readonly_account
