<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00773(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $attr = $req->param('attr');
    if (!preg_match('/^[a-zA-Z0-9._-]+$/', $attr)) {
        return BenchmarkResponse::badRequest('invalid attribute');
    }
    $filter = "($attr=*)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
