<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_group_membership_inject
function ldapi026(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $role = $req->param('role');
    $filter = "(memberOf=cn=$role,ou=groups,dc=example,dc=com)";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_group_membership_inject
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_group_membership_inject
