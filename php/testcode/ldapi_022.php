<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_nested_operator
function ldapi022(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $user = $req->param('user');
    $filter = "(&(uid=$user)(active=TRUE))";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_nested_operator
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_nested_operator
