<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00407(BenchmarkRequest $req): BenchmarkResponse {
    $twig = new Twig\Environment(new Twig\Loader\FilesystemLoader('/views'));
    $sandbox = new Twig\Extension\SandboxExtension(new Twig\Sandbox\SecurityPolicy());
    $twig->addExtension($sandbox);
    $twig->enableSandbox();
    $html = $twig->render($req->param('tpl'), []);
    return BenchmarkResponse::ok($html);
}
