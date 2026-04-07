<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00936(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('q');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render('search.twig', ['q' => $input]);
    return BenchmarkResponse::ok($html);
}
