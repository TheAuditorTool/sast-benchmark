<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00364(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    ldap_bind($conn, 'cn=readonly,dc=example,dc=com', getenv('LDAP_READONLY_PASS'));
    $result = ldap_search($conn, $base, '(objectClass=inetOrgPerson)');
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
