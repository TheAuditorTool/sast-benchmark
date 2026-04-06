<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_bool_operator
function ldapi010(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $group = $req->param('group');
    $filter = "(|(memberOf=" . $group . ")(objectClass=person))"; // vuln-code-snippet vuln-line php_ldapi_bool_operator
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_bool_operator
