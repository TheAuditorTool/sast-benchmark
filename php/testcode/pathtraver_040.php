<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_db_id_serve_content
function pathtraver040(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = getDbConnection();
    $stmt = $db->prepare("SELECT content FROM documents WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch(PDO::FETCH_ASSOC); // vuln-code-snippet safe-line php_pt_db_id_serve_content
    return BenchmarkResponse::ok($row['content'] ?? '');
}
// vuln-code-snippet end php_pt_db_id_serve_content
