<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_read_constant
function ldapi016(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $result = ldap_read($conn, $base, "(objectClass=person)"); // vuln-code-snippet safe-line php_ldapi_read_constant
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_read_constant
