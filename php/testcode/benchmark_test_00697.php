<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00697(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $pdo = getDbConnection();

    $stmt = $pdo->prepare('SELECT bio FROM users WHERE id = ?');
    $stmt->execute([$id]);
    $row = $stmt->fetch();
    $bio = $row['bio'] ?? '';

    $safe = htmlspecialchars($bio, ENT_QUOTES, 'UTF-8');
    $html = '<div class="bio">' . $safe . '</div>';

    return BenchmarkResponse::html($html);
}
