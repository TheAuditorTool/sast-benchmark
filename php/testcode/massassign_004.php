<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_guarded
function massassign004(BenchmarkRequest $req): BenchmarkResponse {
    $guarded = ['is_admin', 'role'];
    $data = array_diff_key($req->postData, array_flip($guarded)); // vuln-code-snippet safe-line php_massassign_guarded
    $pdo = getDbConnection();
    $sets = [];
    foreach ($data as $col => $val) {
        $sets[] = "$col = ?";
    }
    $sql = "UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute(array_merge(array_values($data), [$req->param('id')]));
    return BenchmarkResponse::json(['updated' => true]);
}
// vuln-code-snippet end php_massassign_guarded
