<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00901(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $attr = $req->param('attr');
    $val = $req->param('val');
    $filter = "($attr=$val)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
