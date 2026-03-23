<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_like_concat
function sqli015(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $search = $req->param('search');
    $query = "SELECT * FROM users WHERE name LIKE '%" . $search . "%'"; // vuln-code-snippet vuln-line php_sqli_like_concat
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_like_concat
