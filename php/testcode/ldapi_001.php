<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_search_concat
function ldapi001(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('username');
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $result = ldap_search($conn, $base, "(uid=" . $input . ")"); // vuln-code-snippet vuln-line php_ldapi_search_concat
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_search_concat
