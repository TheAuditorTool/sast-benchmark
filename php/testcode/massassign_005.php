<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_array_merge
function massassign005(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id = ?");
    $stmt->execute([$req->param('id')]);
    $user = $stmt->fetch(PDO::FETCH_ASSOC);
    $user = array_merge($user, $req->postData); // vuln-code-snippet vuln-line php_massassign_array_merge
    $sets = [];
    foreach ($user as $col => $val) {
        $sets[] = "$col = ?";
    }
    $sql = "UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?";
    $upd = $pdo->prepare($sql);
    $upd->execute(array_merge(array_values($user), [$req->param('id')]));
    return BenchmarkResponse::json(['updated' => true]);
}
// vuln-code-snippet end php_massassign_array_merge
