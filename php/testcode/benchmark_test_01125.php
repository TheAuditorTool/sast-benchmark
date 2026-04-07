<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01125(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('name');
    $twig = new Twig\Environment(
        new Twig\Loader\FilesystemLoader('/views'),
        ['autoescape' => 'html']
    );
    $html = $twig->render('page.html.twig', ['name' => $input]);
    return BenchmarkResponse::ok($html);
}
