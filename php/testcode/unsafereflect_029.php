<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_reflect_post_usort_callback
function unsafereflect029(BenchmarkRequest $req): BenchmarkResponse {
    $comparator = $req->post('comparator');
    $arr = [3, 1, 2];
    usort($arr, $comparator); // vuln-code-snippet vuln-line php_reflect_post_usort_callback
    return BenchmarkResponse::ok(implode(',', $arr));
}
// vuln-code-snippet end php_reflect_post_usort_callback
