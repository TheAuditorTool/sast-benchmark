<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_doctrine_dql_concat
function sqli009(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $dql = "SELECT * FROM users u WHERE u.id = " . $id; // vuln-code-snippet vuln-line php_sqli_doctrine_dql_concat
    $result = $pdo->query($dql);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_doctrine_dql_concat
