<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_list_ou_inject
function ldapi029(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $ou = $req->param('ou');
    $result = ldap_list($conn, "ou=$ou,dc=example,dc=com", '(objectClass=*)'); // vuln-code-snippet vuln-line php_ldapi_list_ou_inject
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_list_ou_inject
