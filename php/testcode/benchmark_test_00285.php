<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00285(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = $req->param('uid');
    $safeUid = ldap_escape($uid, '', LDAP_ESCAPE_FILTER);
    $filter = "(uid=" . $safeUid . ")";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
