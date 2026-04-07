<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_stored_filter_multihop
function ldapi031(BenchmarkRequest $req): BenchmarkResponse {
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $pdo = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $pdo->prepare('SELECT filter_fragment FROM saved_searches WHERE id = ?');
    $stmt->execute([$id]);
    $fragment = $stmt->fetchColumn();
    $filter = "(&(objectClass=inetOrgPerson)($fragment))";
    $result = ldap_search($conn, $base, $filter); // vuln-code-snippet vuln-line php_ldapi_stored_filter_multihop
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json(['count' => $entries['count']]);
}
// vuln-code-snippet end php_ldapi_stored_filter_multihop
