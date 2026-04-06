<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_tj_intval_truncation
function typejuggling012(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('user_id');
    $id = intval($input); // vuln-code-snippet vuln-line php_tj_intval_truncation
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id = ?");
    $stmt->execute([$id]);
    $user = $stmt->fetch(\PDO::FETCH_ASSOC);
    return BenchmarkResponse::json($user ?: ['error' => 'not found']);
}
// vuln-code-snippet end php_tj_intval_truncation
