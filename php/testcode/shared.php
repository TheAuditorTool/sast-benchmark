<?php
/**
 * Shared helpers for PHP SAST Benchmark test cases.
 *
 * Each test file includes this file and uses BenchmarkRequest/BenchmarkResponse
 * for a framework-agnostic interface. Test functions receive input via
 * BenchmarkRequest and return BenchmarkResponse.
 *
 * This file is NOT a test case -- it has no vuln-code-snippet annotations.
 */

// ============================================================================
// Request / Response abstractions
// ============================================================================

class BenchmarkRequest
{
    public array $queryParams;
    public array $postData;
    public array $cookies;
    public array $headers;
    public array $files;
    public ?string $body;

    public function __construct(
        array $queryParams = [],
        array $postData = [],
        array $cookies = [],
        array $headers = [],
        array $files = [],
        ?string $body = null
    ) {
        $this->queryParams = $queryParams;
        $this->postData = $postData;
        $this->cookies = $cookies;
        $this->headers = $headers;
        $this->files = $files;
        $this->body = $body;
    }

    /** Get a query parameter ($_GET equivalent). */
    public function param(string $name, string $default = ''): string
    {
        return $this->queryParams[$name] ?? $default;
    }

    /** Get a POST parameter ($_POST equivalent). */
    public function post(string $name, string $default = ''): string
    {
        return $this->postData[$name] ?? $default;
    }

    /** Get a cookie value ($_COOKIE equivalent). */
    public function cookie(string $name, string $default = ''): string
    {
        return $this->cookies[$name] ?? $default;
    }

    /** Get a request header ($_SERVER['HTTP_*'] equivalent). */
    public function header(string $name, string $default = ''): string
    {
        return $this->headers[$name] ?? $default;
    }

    /** Get raw request body (php://input equivalent). */
    public function bodyStr(): string
    {
        return $this->body ?? '';
    }

    /** Get an uploaded file ($_FILES equivalent). */
    public function file(string $name): ?array
    {
        return $this->files[$name] ?? null;
    }

    /** Build from PHP superglobals (for real execution). */
    public static function fromGlobals(): self
    {
        return new self(
            $_GET,
            $_POST,
            $_COOKIE,
            getallheaders() ?: [],
            $_FILES,
            file_get_contents('php://input') ?: null
        );
    }
}

class BenchmarkResponse
{
    public int $status;
    public string $body;
    public array $headers;

    public function __construct(int $status, string $body, array $headers = [])
    {
        $this->status = $status;
        $this->body = $body;
        $this->headers = $headers;
    }

    public static function ok(string $body): self
    {
        return new self(200, $body);
    }

    public static function json($data): self
    {
        return new self(200, json_encode($data), ['Content-Type' => 'application/json']);
    }

    public static function html(string $body): self
    {
        return new self(200, $body, ['Content-Type' => 'text/html; charset=UTF-8']);
    }

    public static function error(string $msg, int $status = 500): self
    {
        return new self($status, $msg);
    }

    public static function badRequest(string $msg): self
    {
        return new self(400, $msg);
    }

    public static function redirect(string $url): self
    {
        return new self(302, '', ['Location' => $url]);
    }
}

// ============================================================================
// Database helpers
// ============================================================================

/** Get a PDO database connection (for test cases that need SQL). */
function getDbConnection(): PDO
{
    static $pdo = null;
    if ($pdo === null) {
        $dsn = getenv('BENCHMARK_DSN') ?: 'sqlite::memory:';
        $user = getenv('BENCHMARK_DB_USER') ?: null;
        $pass = getenv('BENCHMARK_DB_PASS') ?: null;
        $pdo = new PDO($dsn, $user, $pass, [
            PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
            PDO::ATTR_DEFAULT_FETCH_MODE => PDO::FETCH_ASSOC,
        ]);
    }
    return $pdo;
}

/** Get a mysqli connection (for test cases using procedural mysqli). */
function getMysqliConnection(): mysqli
{
    $host = getenv('BENCHMARK_MYSQL_HOST') ?: 'localhost';
    $user = getenv('BENCHMARK_MYSQL_USER') ?: 'root';
    $pass = getenv('BENCHMARK_MYSQL_PASS') ?: '';
    $db = getenv('BENCHMARK_MYSQL_DB') ?: 'benchmark';
    return new mysqli($host, $user, $pass, $db);
}

// ============================================================================
// WordPress-style mocks (for wp_plugin app and WP test cases)
// ============================================================================

class FakeWpdb
{
    private PDO $pdo;
    public string $prefix = 'wp_';

    public function __construct(PDO $pdo)
    {
        $this->pdo = $pdo;
    }

    /** Unsafe query -- no parameter binding. */
    public function query(string $sql)
    {
        return $this->pdo->query($sql);
    }

    /** WordPress-style prepare with sprintf-like placeholders. */
    public function prepare(string $sql, ...$args): string
    {
        $sql = str_replace('%s', "'%s'", $sql);
        return vsprintf($sql, array_map(function ($arg) {
            return addslashes((string)$arg);
        }, $args));
    }

    public function get_var(string $sql)
    {
        $stmt = $this->pdo->query($sql);
        return $stmt ? $stmt->fetchColumn() : null;
    }

    public function get_row(string $sql)
    {
        $stmt = $this->pdo->query($sql);
        return $stmt ? $stmt->fetch(PDO::FETCH_OBJECT) : null;
    }

    public function get_results(string $sql)
    {
        $stmt = $this->pdo->query($sql);
        return $stmt ? $stmt->fetchAll(PDO::FETCH_OBJECT) : [];
    }
}

/** WordPress-style escaping functions. */
function esc_html(string $text): string
{
    return htmlspecialchars($text, ENT_QUOTES, 'UTF-8');
}

function esc_attr(string $text): string
{
    return htmlspecialchars($text, ENT_QUOTES, 'UTF-8');
}

function esc_url(string $url): string
{
    $url = filter_var($url, FILTER_SANITIZE_URL);
    if (!in_array(parse_url($url, PHP_URL_SCHEME), ['http', 'https', ''], true)) {
        return '';
    }
    return htmlspecialchars($url, ENT_QUOTES, 'UTF-8');
}

function wp_kses_post(string $content): string
{
    return strip_tags($content, '<p><br><strong><em><a><ul><ol><li><h1><h2><h3>');
}

function wp_verify_nonce(string $nonce, string $action): bool
{
    return hash_equals(hash_hmac('sha256', $action, 'nonce-secret'), $nonce);
}

function sanitize_text_field(string $str): string
{
    return htmlspecialchars(strip_tags(trim($str)), ENT_QUOTES, 'UTF-8');
}

// ============================================================================
// Laravel-style mocks (for laravel_api app)
// ============================================================================

class FakeDB
{
    private static PDO $pdo;

    public static function setPdo(PDO $pdo): void
    {
        self::$pdo = $pdo;
    }

    /** Unsafe raw query wrapper. */
    public static function raw(string $value): string
    {
        return $value;
    }

    /** Safe parameterized select. */
    public static function select(string $sql, array $bindings = []): array
    {
        $stmt = self::$pdo->prepare($sql);
        $stmt->execute($bindings);
        return $stmt->fetchAll(PDO::FETCH_ASSOC);
    }

    /** Unsafe: selectRaw with concatenation. */
    public static function selectRaw(string $expression): array
    {
        return self::$pdo->query($expression)->fetchAll(PDO::FETCH_ASSOC);
    }
}

// ============================================================================
// Utility
// ============================================================================

/** Check if a string starts with a prefix. */
function str_starts_with_polyfill(string $haystack, string $needle): bool
{
    return strncmp($haystack, $needle, strlen($needle)) === 0;
}
