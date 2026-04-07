<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01242(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->param('username');
    $conn = ldap_connect('ldap://directory.example.com');
    ldap_bind($conn, 'cn=admin,dc=example,dc=com', getenv('LDAP_PASS'));
    $safe = ldap_escape($username, '', LDAP_ESCAPE_FILTER);
    $filter = '(uid=' . $safe . ')';
    $results = ldap_search($conn, 'dc=example,dc=com', $filter);
    $entries = ldap_get_entries($conn, $results);
    ldap_close($conn);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
