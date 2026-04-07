<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_auth_bypass_operator
function ldapi025(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $u = $req->param('u');
    $p = $req->param('p');
    $filter = "(&(uid=$u)(userPassword=$p))";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_auth_bypass_operator
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::ok($entries['count'] > 0 ? 'authenticated' : 'denied');
}
// vuln-code-snippet end php_ldapi_auth_bypass_operator
