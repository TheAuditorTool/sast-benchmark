<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00327(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $policy = new \Twig\Sandbox\SecurityPolicy(
        ['if', 'for'],
        ['upper', 'lower', 'escape'],
        [],
        [],
        []
    );
    $sandbox = new \Twig\Extension\SandboxExtension($policy, true);
    $loader = new \Twig\Loader\ArrayLoader(['user' => $template]);
    $twig = new \Twig\Environment($loader);
    $twig->addExtension($sandbox);
    $output = $twig->render('user', ['name' => 'World']);
    return BenchmarkResponse::html($output);
}
