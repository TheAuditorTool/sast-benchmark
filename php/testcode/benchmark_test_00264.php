<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00264(BenchmarkRequest $req): BenchmarkResponse {
    $user = $req->param('user');
    $viewFile = __DIR__ . '/views/profile.blade.php';
    $data = ['user' => htmlspecialchars($user, ENT_QUOTES, 'UTF-8')];
    ob_start();
    extract($data);
    include $viewFile;
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
