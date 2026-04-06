<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_preg_template
function ssti011(BenchmarkRequest $req): BenchmarkResponse {
    $template = $req->post('template');
    $data = ['name' => 'World', 'year' => '2026'];
    $output = preg_replace_callback('/\{\{(\w+)\}\}/', function($m) use ($data, $template) {
        return $data[$m[1]] ?? eval('return ' . $m[1] . ';'); // vuln-code-snippet vuln-line php_ssti_preg_template
    }, $template);
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_preg_template
