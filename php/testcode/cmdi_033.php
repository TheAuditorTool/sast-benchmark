<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_cmdi_db_multihop
function cmdi033(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $db->query("SELECT cmd FROM scheduled_tasks WHERE id = $id");
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    $output = shell_exec($row['cmd']); // vuln-code-snippet vuln-line php_cmdi_db_multihop
    return BenchmarkResponse::ok($output ?? '');
}
// vuln-code-snippet end php_cmdi_db_multihop
