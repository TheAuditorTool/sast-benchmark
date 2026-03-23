<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_fillable
function massassign002(BenchmarkRequest $req): BenchmarkResponse {
    $fillable = ['name', 'email'];
    $data = array_intersect_key($req->postData, array_flip($fillable)); // vuln-code-snippet safe-line php_massassign_fillable
    $pdo = getDbConnection();
    $cols = implode(', ', array_keys($data));
    $placeholders = implode(', ', array_fill(0, count($data), '?'));
    $stmt = $pdo->prepare("INSERT INTO users ($cols) VALUES ($placeholders)");
    $stmt->execute(array_values($data));
    return BenchmarkResponse::json(['created' => true]);
}
// vuln-code-snippet end php_massassign_fillable
