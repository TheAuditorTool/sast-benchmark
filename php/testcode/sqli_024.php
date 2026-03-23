<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_table_name_allowlist
function sqli024(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $tableName = $req->param('table');
    $allowed = ['users', 'posts', 'comments'];
    if (!in_array($tableName, $allowed, true)) { // vuln-code-snippet safe-line php_sqli_table_name_allowlist
        return BenchmarkResponse::badRequest("Invalid table name");
    }
    $query = "SELECT * FROM " . $tableName;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_table_name_allowlist
