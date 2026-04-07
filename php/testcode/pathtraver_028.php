<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_db_fopen_multihop
function pathtraver028(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $db->query("SELECT file_path FROM documents WHERE id = $id");
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    $content = file_get_contents($row['file_path']); // vuln-code-snippet vuln-line php_pt_db_fopen_multihop
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_db_fopen_multihop
