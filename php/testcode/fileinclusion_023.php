<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_db_config_second_order
function fileinclusion023(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('uid');
    $stmt = $req->db()->prepare('SELECT template_path FROM user_settings WHERE user_id = ?');
    $stmt->execute([$userId]);
    $tplPath = $stmt->fetchColumn();
    include $tplPath; // vuln-code-snippet vuln-line php_fi_db_config_second_order
    return BenchmarkResponse::ok('Template rendered');
}
// vuln-code-snippet end php_fi_db_config_second_order
