<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00957(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $name = $req->param('name');
    $filter = "(cn=" . $name . ")";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
