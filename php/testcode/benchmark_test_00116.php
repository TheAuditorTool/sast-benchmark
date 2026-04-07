<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00116(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('template');
    $twig = new Twig\Environment(new Twig\Loader\ArrayLoader());
    $tmpl = $twig->createTemplate($input);
    $html = $tmpl->render([]);
    return BenchmarkResponse::ok($html);
}
