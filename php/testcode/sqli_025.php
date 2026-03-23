<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_stored_taint
function sqli025(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT username FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $username = $stmt->fetchColumn();
    $result = $pdo->query("SELECT * FROM logs WHERE actor = '" . $username . "'"); // vuln-code-snippet vuln-line php_sqli_stored_taint
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_stored_taint
