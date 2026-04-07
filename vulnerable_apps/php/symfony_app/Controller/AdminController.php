<?php
// Symfony-style Admin Controller
require_once __DIR__ . '/../../../testcode/shared.php';

// vuln-code-snippet start sy_ldapi_search
function sy_ldapi_search(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->param('username');
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $result = ldap_search($conn, $base, "(uid=" . $username . ")"); // vuln-code-snippet vuln-line sy_ldapi_search
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end sy_ldapi_search

// vuln-code-snippet start sy_ldapi_escaped
function sy_ldapi_escaped(BenchmarkRequest $req): BenchmarkResponse {
    $username = $req->param('username');
    $conn = ldap_connect('ldap://localhost');
    $base = 'dc=example,dc=com';
    $safe = ldap_escape($username, '', LDAP_ESCAPE_FILTER); // vuln-code-snippet safe-line sy_ldapi_escaped
    $result = ldap_search($conn, $base, "(uid=" . $safe . ")");
    $entries = ldap_get_entries($conn, $result);
    return BenchmarkResponse::json($entries);
}
// vuln-code-snippet end sy_ldapi_escaped

// vuln-code-snippet start sy_vv_dynamic
function sy_vv_dynamic(BenchmarkRequest $req): BenchmarkResponse {
    $settingName = $req->post('name');
    $value = $req->post('value');
    $$settingName = $value; // vuln-code-snippet vuln-line sy_vv_dynamic
    return BenchmarkResponse::ok("Set $settingName");
}
// vuln-code-snippet end sy_vv_dynamic

// vuln-code-snippet start sy_vv_array
function sy_vv_array(BenchmarkRequest $req): BenchmarkResponse {
    $settings = [];
    $name = $req->post('name');
    $value = $req->post('value');
    $settings[$name] = $value; // vuln-code-snippet safe-line sy_vv_array
    return BenchmarkResponse::ok("Set $name");
}
// vuln-code-snippet end sy_vv_array

// vuln-code-snippet start sy_xxe_form
function sy_xxe_form(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml); // vuln-code-snippet vuln-line sy_xxe_form
    $name = $doc->getElementsByTagName('name')->item(0)->nodeValue;
    return BenchmarkResponse::ok($name);
}
// vuln-code-snippet end sy_xxe_form

// vuln-code-snippet start sy_xxe_safe
function sy_xxe_safe(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml, LIBXML_NOENT | LIBXML_NONET); // vuln-code-snippet safe-line sy_xxe_safe
    $name = $doc->getElementsByTagName('name')->item(0)->nodeValue;
    return BenchmarkResponse::ok($name);
}
// vuln-code-snippet end sy_xxe_safe

// vuln-code-snippet start sy_pt_asset
function sy_pt_asset(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $data = file_get_contents("assets/" . $file); // vuln-code-snippet vuln-line sy_pt_asset
    return BenchmarkResponse::ok($data);
}
// vuln-code-snippet end sy_pt_asset

// vuln-code-snippet start sy_pt_safe
function sy_pt_safe(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->param('file');
    $base = realpath(__DIR__ . '/../assets');
    $full = realpath($base . '/' . $file);
    if ($full === false || strpos($full, $base) !== 0) { // vuln-code-snippet safe-line sy_pt_safe
        return BenchmarkResponse::badRequest('Invalid path');
    }
    $data = file_get_contents($full);
    return BenchmarkResponse::ok($data);
}
// vuln-code-snippet end sy_pt_safe
