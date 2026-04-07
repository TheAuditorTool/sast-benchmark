<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00956(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $uid = $req->param('uid');
    $metacharacters = ['\\', '*', '(', ')', "\0"];
    $replacements = ['\\5c', '\\2a', '\\28', '\\29', '\\00'];
    $safeUid = str_replace($metacharacters, $replacements, $uid);
    $filter = "(uid=" . $safeUid . ")";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
