<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00684(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = new PDO('sqlite:/tmp/templates.db');
    $stmt = $db->prepare('SELECT body FROM templates WHERE id = ?');
    $stmt->execute([$id]);
    $tpl = $stmt->fetchColumn();
    $twig = new Twig\Environment(new Twig\Loader\ArrayLoader());
    $html = $twig->createTemplate($tpl)->render([]);
    return BenchmarkResponse::ok($html);
}
