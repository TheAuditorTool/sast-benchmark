<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_enum_attribute
function ldapi041(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $choice = (int)$req->param('choice');
    $attr = ['uid', 'cn', 'mail'][$choice] ?? 'uid'; // vuln-code-snippet safe-line php_ldapi_enum_attribute
    $filter = "($attr=*)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_enum_attribute
