<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_db_row_second_order
function extract023(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('uid');
    $stmt = $req->db()->prepare('SELECT * FROM user_prefs WHERE user_id = ?');
    $stmt->execute([$userId]);
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    extract($row); // vuln-code-snippet vuln-line php_extract_db_row_second_order
    return BenchmarkResponse::ok("theme=$theme");
}
// vuln-code-snippet end php_extract_db_row_second_order
