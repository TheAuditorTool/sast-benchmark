<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00543(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('tpl');
    $loader = new Twig\Loader\ArrayLoader(['user' => $input]);
    $env = new Twig\Environment($loader);
    $html = $env->render('user', []);
    return BenchmarkResponse::ok($html);
}
