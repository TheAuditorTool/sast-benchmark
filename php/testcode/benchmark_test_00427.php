<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00427(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\ArrayLoader([]);
    $twig = new \Twig\Environment($loader);
    $template = $req->post('template');
    $output = $twig->createTemplate($template)->render([]);
    return BenchmarkResponse::html($output);
}
