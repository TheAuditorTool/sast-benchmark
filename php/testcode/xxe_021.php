<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xslt_user_stylesheet
function xxe021(BenchmarkRequest $req): BenchmarkResponse {
    $xslPath = $req->param('xsl');
    $xsl = new DOMDocument();
    $xsl->load($xslPath); // vuln-code-snippet vuln-line php_xxe_xslt_user_stylesheet
    $proc = new XSLTProcessor();
    $proc->importStylesheet($xsl);
    return BenchmarkResponse::ok('imported');
}
// vuln-code-snippet end php_xxe_xslt_user_stylesheet
