<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00270(BenchmarkRequest $req): BenchmarkResponse {
    $tpl = constant('DEFAULT_TEMPLATE');
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $html = $twig->render($tpl, ['data' => $req->param('q')]);
    return BenchmarkResponse::ok($html);
}
