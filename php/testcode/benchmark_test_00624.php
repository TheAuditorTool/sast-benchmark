<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00624(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('prefs');
    parse_str($input, $parsed);
    $allowed = array_intersect_key($parsed, array_flip(['lang', 'theme']));
    $twig = $req->twig();
    $html = $twig->render('page.twig', $allowed);
    return BenchmarkResponse::ok($html);
}
