<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_orm_create
function massassign008(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $columns = implode(', ', array_keys($data));
    $placeholders = implode(', ', array_fill(0, count($data), '?'));
    $stmt = $pdo->prepare("INSERT INTO users ($columns) VALUES ($placeholders)"); // vuln-code-snippet vuln-line php_massassign_orm_create
    $stmt->execute(array_values($data));
    return BenchmarkResponse::ok('User created');
}
// vuln-code-snippet end php_massassign_orm_create
