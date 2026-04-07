<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00666(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('username');
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $safe = ldap_escape($input, '', LDAP_ESCAPE_FILTER);
    $result = ldap_search($conn, $base, "(uid=" . $safe . ")");
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
