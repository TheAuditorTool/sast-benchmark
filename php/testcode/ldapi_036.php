<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_intval_uid_safe
function ldapi036(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = intval($req->param('uid')); // vuln-code-snippet safe-line php_ldapi_intval_uid_safe
    $filter = "(uidNumber=$uid)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_intval_uid_safe
