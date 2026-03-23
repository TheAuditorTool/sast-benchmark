<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_search_escaped
function ldapi002(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('username');
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $safe = ldap_escape($input, '', LDAP_ESCAPE_FILTER); // vuln-code-snippet safe-line php_ldapi_search_escaped
    $result = ldap_search($conn, $base, "(uid=" . $safe . ")");
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end php_ldapi_search_escaped
