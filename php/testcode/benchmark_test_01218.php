<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01218(BenchmarkRequest $req): BenchmarkResponse {
    $loader = new \Twig\Loader\ArrayLoader([]);
    $twig = new \Twig\Environment($loader);
    $greeting = $req->param('greeting');
    $compiled = $twig->createTemplate($greeting);
    $output = $compiled->render(['user' => 'world']);
    return BenchmarkResponse::html($output);
}
