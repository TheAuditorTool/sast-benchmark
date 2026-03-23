<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_basedn_injection
function ldapi005(BenchmarkRequest $req): BenchmarkResponse {
    $base = $req->param('base');
    $conn = ldap_connect('ldap://localhost');
    $filter = "(objectClass=person)";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_basedn_injection
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_basedn_injection
