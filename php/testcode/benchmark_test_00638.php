<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00638(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = $req->param('template_id');
    $stmt = $pdo->prepare("SELECT path FROM templates WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    $templatePath = $row['path'];
    include($templatePath);
    return BenchmarkResponse::ok("template rendered");
}
