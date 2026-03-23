<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_update_all
function massassign003(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $sets = [];
    foreach ($data as $col => $val) {
        $sets[] = "$col = ?";
    }
    $sql = "UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute(array_merge(array_values($data), [$req->param('id')])); // vuln-code-snippet vuln-line php_massassign_update_all
    return BenchmarkResponse::json(['updated' => true]);
}
// vuln-code-snippet end php_massassign_update_all
