<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00654(BenchmarkRequest $req): BenchmarkResponse {
    $userId = (int) $req->param('uid');
    $stmt = $req->db()->prepare('SELECT * FROM user_prefs WHERE user_id = ?');
    $stmt->execute([$userId]);
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    extract($row);
    return BenchmarkResponse::ok("theme=$theme");
}
