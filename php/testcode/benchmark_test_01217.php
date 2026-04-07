<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01217(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\ArrayLoader([]);
    $twig = new \Twig\Environment($loader);
    $template = $req->param('template');
    $rendered = $twig->createTemplate($template)->render([]);
    return BenchmarkResponse::html($rendered);
}
