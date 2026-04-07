<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00377(BenchmarkRequest $req): BenchmarkResponse {
    $tplStr = file_get_contents(getenv('TEMPLATE_PATH'));
    $tplStr .= $req->param('extra');
    $twig = new Twig\Environment(new Twig\Loader\ArrayLoader(['main' => $tplStr]));
    echo $twig->render('main', []);
    return BenchmarkResponse::ok('rendered');
}
