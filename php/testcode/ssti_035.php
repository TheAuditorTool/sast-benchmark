<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_twig_sandbox_enabled
function ssti035(BenchmarkRequest $req): BenchmarkResponse {
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $sandbox = new Twig\Extension\SandboxExtension(new Twig\Sandbox\SecurityPolicy());
    $twig->addExtension($sandbox);
    $twig->enableSandbox(); // vuln-code-snippet safe-line php_ssti_twig_sandbox_enabled
    $html = $twig->render($req->param('tpl'), []);
    return BenchmarkResponse::ok($html);
}
// vuln-code-snippet end php_ssti_twig_sandbox_enabled
