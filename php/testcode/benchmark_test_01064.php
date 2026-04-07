<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01064(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $v = ldap_escape($req->param('v'), '', LDAP_ESCAPE_FILTER);
    $filter = "(uid=$v)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
