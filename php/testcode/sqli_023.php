<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_table_name
function sqli023(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $tableName = $req->param('table');
    $query = "SELECT * FROM " . $tableName; // vuln-code-snippet vuln-line php_sqli_table_name
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_table_name
