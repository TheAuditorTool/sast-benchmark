<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00530(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $pdo = getDbConnection();

    $stmt = $pdo->prepare('SELECT bio FROM users WHERE id = ?');
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    $bio = $row['bio'] ?? '';

    $html = '<div class="bio">' . $bio . '</div>';

    return BenchmarkResponse::html($html);
}
