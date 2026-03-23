<?php
// vuln_blog - Database configuration and helpers

// vuln-code-snippet start vb_hardcoded_db_pass
function getDbConnectionHardcoded(): PDO {
    $dsn = 'mysql:host=localhost;dbname=vuln_blog';
    $pdo = new PDO($dsn, "root", "BlogP@ss2024!"); // vuln-code-snippet vuln-line vb_hardcoded_db_pass
    $pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
    return $pdo;
}
// vuln-code-snippet end vb_hardcoded_db_pass

// vuln-code-snippet start vb_hardcoded_env
function getDbConnectionEnv(): PDO {
    $host = getenv('DB_HOST');
    $name = getenv('DB_NAME');
    $user = getenv('DB_USER');
    $pass = getenv('DB_PASS');
    $dsn = "mysql:host=$host;dbname=$name";
    $pdo = new PDO($dsn, $user, $pass); // vuln-code-snippet safe-line vb_hardcoded_env
    $pdo->setAttribute(PDO::ATTR_ERRMODE, PDO::ERRMODE_EXCEPTION);
    return $pdo;
}
// vuln-code-snippet end vb_hardcoded_env
