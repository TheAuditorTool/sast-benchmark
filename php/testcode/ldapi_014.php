<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_filter_builder
function ldapi014(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = $req->param('uid');
    $safeUid = ldap_escape($uid, '', LDAP_ESCAPE_FILTER);
    $filter = "(uid=" . $safeUid . ")"; // vuln-code-snippet safe-line php_ldapi_filter_builder
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_filter_builder
