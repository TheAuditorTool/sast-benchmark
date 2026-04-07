<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00632(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int) $req->param('id');
    $stmt = $req->db()->prepare('SELECT title, body FROM posts WHERE id = ?');
    $stmt->execute([$id]);
    $row   = $stmt->fetch(PDO::FETCH_ASSOC);
    $title = $row['title'];
    $body  = $row['body'];
    return BenchmarkResponse::json(['title' => $title, 'body' => $body]);
}
