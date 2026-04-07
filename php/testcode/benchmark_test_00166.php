<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00166(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uidNum = (int)$req->param('uidNumber');
    $filter = "(uidNumber=" . $uidNum . ")";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
