<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00087(BenchmarkRequest $req): BenchmarkResponse {
    $userPath = $req->param('path');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render('../' . $userPath);
    return BenchmarkResponse::ok($html);
}
