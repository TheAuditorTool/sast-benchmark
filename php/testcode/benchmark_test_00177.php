<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00177(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $user = ldap_escape($req->param('user'), '', LDAP_ESCAPE_FILTER);
    $filter = "(uid=$user)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
