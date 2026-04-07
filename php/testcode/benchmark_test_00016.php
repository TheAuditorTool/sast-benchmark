<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00016(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\FilesystemLoader(__DIR__ . '/templates');
    $twig = new \Twig\Environment($loader, ['autoescape' => 'html']);
    $output = $twig->render('greeting.html.twig', ['name' => $req->param('name')]);
    return BenchmarkResponse::html($output);
}
