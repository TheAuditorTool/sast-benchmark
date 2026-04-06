<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_sandbox
function ssti012(BenchmarkRequest $req): BenchmarkResponse {
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
    $output = $twig->render('user', ['name' => 'World']); // vuln-code-snippet safe-line php_ssti_twig_sandbox
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_twig_sandbox
