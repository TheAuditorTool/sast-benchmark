<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00674(BenchmarkRequest $req): BenchmarkResponse {
    $twig = new Twig\Environment(
        new Twig\Loader\FilesystemLoader('/views'),
        ['autoescape' => 'html', 'strict_variables' => true]
    );
    $html = $twig->render('template.twig', ['q' => $req->param('q')]);
    return BenchmarkResponse::ok($html);
}
