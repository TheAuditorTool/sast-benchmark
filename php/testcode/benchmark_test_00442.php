<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00442(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $scope = (int)$req->param('scope');
    $filter = "(uid=" . $req->param('uid') . ")";
    $result = ldap_search($conn, $base, $filter, [], 0, 0, 0, $scope);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
