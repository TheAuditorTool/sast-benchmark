<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_stored_validated
function sqli026(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT age FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $age = $stmt->fetchColumn();
    $safeAge = intval($age); // vuln-code-snippet safe-line php_sqli_stored_validated
    $result = $pdo->query("SELECT * FROM stats WHERE age = " . $safeAge);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_stored_validated
