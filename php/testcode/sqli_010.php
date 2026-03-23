<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_doctrine_param
function sqli010(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $stmt = $pdo->prepare("SELECT * FROM users u WHERE u.id = :id"); // vuln-code-snippet safe-line php_sqli_doctrine_param
    $stmt->execute(['id' => $id]);
    $rows = $stmt->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_doctrine_param
