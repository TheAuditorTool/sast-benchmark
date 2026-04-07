<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_ldapi_saml_no_ldap_auth
function ldapi046(BenchmarkRequest $req): BenchmarkResponse {
    $samlResponse = $req->post('SAMLResponse');
    $assertion = base64_decode($samlResponse);
    $pubKey = openssl_pkey_get_public(getenv('SAML_PUBLIC_KEY'));
    $doc = new DOMDocument();
    $doc->loadXML($assertion);
    $sig = $doc->getElementsByTagNameNS('http://www.w3.org/2000/09/xmldsig#', 'SignatureValue')->item(0);
    $valid = openssl_verify($assertion, base64_decode($sig->nodeValue), $pubKey, OPENSSL_ALGO_SHA256); // vuln-code-snippet safe-line php_ldapi_saml_no_ldap_auth
    if ($valid !== 1) {
        return BenchmarkResponse::badRequest('invalid saml assertion');
    }
    return BenchmarkResponse::ok('authenticated via saml');
}
// vuln-code-snippet end php_ldapi_saml_no_ldap_auth
