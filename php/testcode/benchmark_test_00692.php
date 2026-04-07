<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00692(BenchmarkRequest $req): BenchmarkResponse {
    $base = $req->param('base');
    $conn = ldap_connect('ldap://localhost');
    $filter = "(objectClass=person)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
