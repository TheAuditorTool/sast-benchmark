<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00570(BenchmarkRequest $req): BenchmarkResponse {
    $m = new Mustache_Engine(['loader' => new Mustache_Loader_FilesystemLoader('/views')]);
    $tmpl = $m->loadTemplate('user_card');
    $html = $tmpl->render(['name' => $req->param('name')]);
    return BenchmarkResponse::ok($html);
}
