<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00969(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $dnMap = [
        'sales' => 'ou=sales,dc=example,dc=com',
        'eng' => 'ou=engineering,dc=example,dc=com',
        'hr' => 'ou=hr,dc=example,dc=com',
    ];
    $id = $req->param('id');
    $dn = $dnMap[$id] ?? null;
    if (!$dn) {
        return BenchmarkResponse::badRequest('unknown department');
    }
    $result = ldap_search($conn, $dn, '(objectClass=*)');
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
