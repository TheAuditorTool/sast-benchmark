<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00173(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->param('page');
    $allowed = ['home', 'about', 'contact'];
    if (!in_array($name, $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid');
    }
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render($name . '.twig', []);
    return BenchmarkResponse::ok($html);
}
