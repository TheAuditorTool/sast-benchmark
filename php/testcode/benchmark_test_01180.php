<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01180(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->param('template');
    $allowed = ['invoice', 'receipt', 'statement'];
    if (!in_array($template, $allowed, true)) {
        return BenchmarkResponse::badRequest('unknown template');
    }
    $content = file_get_contents('/app/templates/' . $template . '.html');
    return BenchmarkResponse::html($content);
}
