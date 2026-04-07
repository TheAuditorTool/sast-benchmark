<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_constant_filter
function ldapi042(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $result = ldap_search($conn, $base, '(objectClass=inetOrgPerson)'); // vuln-code-snippet safe-line php_ldapi_constant_filter
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_constant_filter
