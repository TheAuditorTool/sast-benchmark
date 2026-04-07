<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00355(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $map = [1 => 'home.twig', 2 => 'about.twig'];
    $tpl = $map[$id] ?? 'default.twig';
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render($tpl, ['name' => $req->param('name')]);
    return BenchmarkResponse::ok($html);
}
