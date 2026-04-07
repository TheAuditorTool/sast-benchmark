<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00071(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('module');
    $safe = basename($file);
    $ext = pathinfo($safe, PATHINFO_EXTENSION);
    if ($ext !== 'php') {
        return BenchmarkResponse::badRequest("only .php modules allowed");
    }
    $allowedModules = ['utils.php', 'helpers.php', 'config.php'];
    if (!in_array($safe, $allowedModules, true)) {
        return BenchmarkResponse::badRequest("module not in allowlist");
    }
    include("modules/" . $safe);
    return BenchmarkResponse::ok("module loaded");
}
