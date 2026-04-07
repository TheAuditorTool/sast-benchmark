<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_wildcard_enumerate
function ldapi027(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $q = $req->param('q');
    $filter = "(cn=$q*)";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_wildcard_enumerate
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_wildcard_enumerate
