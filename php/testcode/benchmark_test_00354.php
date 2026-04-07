<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00354(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $result = ldap_read($conn, $base, "(objectClass=person)");
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
