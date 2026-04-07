<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00680(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $role = $req->param('role');
    $filter = "(memberOf=cn=$role,ou=groups,dc=example,dc=com)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
