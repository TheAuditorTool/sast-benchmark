<?php
require_once __DIR__ . '/shared.php';

define('LDAP_BASE_DN', 'dc=example,dc=com');

// vuln-code-snippet start php_ldapi_basedn_constant
function ldapi006(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $username = $req->param('username');
    $safeFilter = "(uid=" . ldap_escape($username, '', LDAP_ESCAPE_FILTER) . ")";
    $result = ldap_search($conn, LDAP_BASE_DN, $safeFilter); // vuln-code-snippet safe-line php_ldapi_basedn_constant
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_basedn_constant
