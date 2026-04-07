<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00231(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $data = ['name' => 'World', 'year' => '2026'];
    $output = preg_replace_callback('/\{\{(\w+)\}\}/', function($m) use ($data, $template) {
        return $data[$m[1]] ?? eval('return ' . $m[1] . ';');
    }, $template);
    return BenchmarkResponse::html($output);
}
