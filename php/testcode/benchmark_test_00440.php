<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00440(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('fn');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $filter = new Twig\TwigFilter('custom', $input);
    $twig->addFilter($filter);
    $html = $twig->render('base.twig', []);
    return BenchmarkResponse::ok($html);
}
