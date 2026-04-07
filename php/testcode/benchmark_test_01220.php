<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01220(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\FilesystemLoader('/app/templates');
    $twig = new \Twig\Environment($loader);
    $message = $req->param('message');
    $output = $twig->render('notification.html', ['message' => $message]);
    return BenchmarkResponse::html($output);
}
