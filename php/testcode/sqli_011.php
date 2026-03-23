<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_sprintf_format
function sqli011(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $query = sprintf("SELECT * FROM users WHERE id = '%s'", $id); // vuln-code-snippet vuln-line php_sqli_sprintf_format
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_sprintf_format
