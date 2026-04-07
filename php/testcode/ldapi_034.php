<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_escape_filter_flag
function ldapi034(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $user = ldap_escape($req->param('user'), '', LDAP_ESCAPE_FILTER); // vuln-code-snippet safe-line php_ldapi_escape_filter_flag
    $filter = "(uid=$user)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_escape_filter_flag
