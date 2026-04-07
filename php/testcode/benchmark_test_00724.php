<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00724(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('uid');
    $stmt = $req->db()->prepare('SELECT template_path FROM user_settings WHERE user_id = ?');
    $stmt->execute([$userId]);
    $tplPath = $stmt->fetchColumn();
    include $tplPath;
    return BenchmarkResponse::ok('Template rendered');
}
