<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_id_to_path_lookup
function pathtraver032(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = getDbConnection();
    $stmt = $db->prepare("SELECT stored_path FROM static_files WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    $content = file_get_contents($row['stored_path']); // vuln-code-snippet safe-line php_pt_id_to_path_lookup
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_id_to_path_lookup
