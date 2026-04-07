<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_utf8_encoded_wildcard
function ldapi024(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $query = urldecode($req->param('q'));
    $filter = "(cn=$query)";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_utf8_encoded_wildcard
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_utf8_encoded_wildcard
