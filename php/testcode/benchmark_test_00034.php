<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00034(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $ou = $req->param('ou');
    $result = ldap_list($conn, "ou=$ou,dc=example,dc=com", '(objectClass=*)');
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
