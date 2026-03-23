<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_create_all
function massassign001(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $cols = implode(', ', array_keys($data));
    $placeholders = implode(', ', array_fill(0, count($data), '?'));
    $stmt = $pdo->prepare("INSERT INTO users ($cols) VALUES ($placeholders)");
    $stmt->execute(array_values($data)); // vuln-code-snippet vuln-line php_massassign_create_all
    return BenchmarkResponse::json(['created' => true]);
}
// vuln-code-snippet end php_massassign_create_all
