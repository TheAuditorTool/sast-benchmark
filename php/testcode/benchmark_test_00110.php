<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00110(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $user = $req->param('user');
    $filter = "(&(uid=$user)(active=TRUE))";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
