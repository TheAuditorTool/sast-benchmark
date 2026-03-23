<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_multihop
function fileinclusion015(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('template_id');
    $stmt = $pdo->prepare("SELECT path FROM templates WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    $templatePath = $row['path'];
    include($templatePath); // vuln-code-snippet vuln-line php_fi_multihop
    return BenchmarkResponse::ok("template rendered");
}
// vuln-code-snippet end php_fi_multihop
