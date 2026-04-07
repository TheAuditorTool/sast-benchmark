<?php
require_once __DIR__ . '/shared.php';

define('LDAP_BASE_DN', 'dc=example,dc=com');

function benchmarkTest00971(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $username = $req->param('username');
    $safeFilter = "(uid=" . ldap_escape($username, '', LDAP_ESCAPE_FILTER) . ")";
    $result = ldap_search($conn, LDAP_BASE_DN, $safeFilter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
