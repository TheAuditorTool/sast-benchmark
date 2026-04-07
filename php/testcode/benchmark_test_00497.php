<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00497(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\FilesystemLoader('/templates');
    $twig = new \Twig\Environment($loader);
    $name = $req->param('name');
    $output = $twig->render('contact.html.twig', ['name' => $name]);
    return BenchmarkResponse::html($output);
}
