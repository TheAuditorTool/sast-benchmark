<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_sqli_multihop_sanitized
function sanitizeForQuery(string $id): int {
    return intval($id);
}

function sqli018(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('id');
    $safeId = sanitizeForQuery($id); // vuln-code-snippet safe-line php_sqli_multihop_sanitized
    $query = "SELECT * FROM users WHERE id = " . $safeId;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
// vuln-code-snippet end php_sqli_multihop_sanitized
