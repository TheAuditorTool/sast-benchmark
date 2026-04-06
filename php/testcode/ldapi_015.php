<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_intval_uid
function ldapi015(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uidNum = (int)$req->param('uidNumber');
    $filter = "(uidNumber=" . $uidNum . ")"; // vuln-code-snippet safe-line php_ldapi_intval_uid
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_intval_uid
