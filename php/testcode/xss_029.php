<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_js_template_literal
function xss029(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('val');
    $html = "<script>var x = `$input`</script>"; // vuln-code-snippet vuln-line php_xss_js_template_literal
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_js_template_literal
