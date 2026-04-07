<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01137(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $loader = new \Twig\Loader\ArrayLoader(['dynamic' => $template]);
    $twig = new \Twig\Environment($loader);
    $output = $twig->render('dynamic', ['name' => 'World']);
    return BenchmarkResponse::html($output);
}
