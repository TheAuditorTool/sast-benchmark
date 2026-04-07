<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00155(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('username');
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $result = ldap_search($conn, $base, "(uid=" . $input . ")");
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
