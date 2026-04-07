<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00237(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $userId = (int) $req->param('user_id');
    $stmt = $db->prepare("SELECT redirect_url FROM user_prefs WHERE user_id = ?");
    $stmt->execute([$userId]);
    $url = $stmt->fetchColumn();
    header('Location: ' . $url);
    return BenchmarkResponse::ok('');
}
