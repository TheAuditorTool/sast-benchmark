<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00621(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $attr = $req->param('attr');
    $allowed = ['cn', 'mail', 'uid', 'telephoneNumber'];
    if (!in_array($attr, $allowed, true)) {
        return BenchmarkResponse::badRequest('Invalid attribute');
    }
    $result = ldap_search($conn, $base, "(objectClass=person)", [$attr]);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
