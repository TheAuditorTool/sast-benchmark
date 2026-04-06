<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_sprintf_filter
function ldapi012(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = $req->param('uid');
    $filter = sprintf("(uid=%s)", $uid); // vuln-code-snippet vuln-line php_ldapi_sprintf_filter
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_sprintf_filter
