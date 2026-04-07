<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00894(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = intval($req->param('uid'));
    $filter = "(uidNumber=$uid)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
