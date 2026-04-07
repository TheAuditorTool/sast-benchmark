<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00693(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $encoded = $req->param('q');
    $decoded = base64_decode($encoded);
    $filter = "(uid=$decoded)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
