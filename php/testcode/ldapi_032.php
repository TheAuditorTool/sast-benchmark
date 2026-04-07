<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_scope_inject
function ldapi032(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $scope = (int)$req->param('scope');
    $filter = "(uid=" . $req->param('uid') . ")";
    $result = ldap_search($conn, $base, $filter, [], 0, 0, 0, $scope); // vuln-code-snippet vuln-line php_ldapi_scope_inject
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_scope_inject
