<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_ctype_alnum_username
function ldapi040(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $u = $req->param('u');
    if (!ctype_alnum($u)) {
        return BenchmarkResponse::badRequest('invalid username');
    }
    $filter = "(uid=$u)";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet safe-line php_ldapi_ctype_alnum_username
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_ctype_alnum_username
