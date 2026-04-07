<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00373(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $email = $req->param('email');
    if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
        return BenchmarkResponse::badRequest('invalid email');
    }
    $filter = "(mail=$email)";
    $result = ldap_search($conn, $base, $filter);
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
