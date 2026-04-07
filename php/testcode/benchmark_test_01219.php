<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01219(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\FilesystemLoader('/app/templates');
    $twig = new \Twig\Environment($loader);
    $name = $req->param('name');
    $rendered = $twig->render('greeting.html', ['name' => $name]);
    return BenchmarkResponse::html($rendered);
}
