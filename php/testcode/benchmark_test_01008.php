<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01008(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $group = $req->param('group');
    $filter = "(|(memberOf=" . $group . ")(objectClass=person))";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
