<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ssti_blade_compile
function ssti003(BenchmarkRequest $req): BenchmarkResponse {
    $content = $req->post('content');
    $compiled = preg_replace('/\{\{(.+?)\}\}/', '<?php echo $1; ?>', $content);
    ob_start();
    eval('?>' . $compiled); // vuln-code-snippet vuln-line php_ssti_blade_compile
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
// vuln-code-snippet end php_ssti_blade_compile
