<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00419(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    ldap_start_tls($conn);
    ldap_bind($conn, 'cn=readonly,dc=example,dc=com', getenv('LDAP_READONLY_PASS'));
    $v = ldap_escape($req->param('v'), '', LDAP_ESCAPE_FILTER);
    $result = ldap_search($conn, $base, "(uid=$v)");
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
