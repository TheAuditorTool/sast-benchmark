# Shared helpers for Ruby SAST Benchmark test cases.
#
# Each test file requires this file and uses BenchmarkRequest/BenchmarkResponse
# for a framework-agnostic interface. Test functions receive input via
# BenchmarkRequest and return BenchmarkResponse.
#
# This file is NOT a test case -- it has no vuln-code-snippet annotations.

require 'json'

begin
  require 'sqlite3'
rescue LoadError
  # sqlite3 gem not available -- tests are syntax-checked, not executed
end

begin
  require 'pg'
rescue LoadError
  # pg gem not available -- tests are syntax-checked, not executed
end

begin
  require 'net/ldap'
rescue LoadError
  # net-ldap gem not available
end

# ============================================================================
# Request / Response abstractions
# ============================================================================

class BenchmarkRequest
  attr_accessor :query_params, :post_data, :cookies, :headers, :files, :body

  def initialize(query_params: {}, post_data: {}, cookies: {}, headers: {}, files: {}, body: nil)
    @query_params = query_params
    @post_data = post_data
    @cookies = cookies
    @headers = headers
    @files = files
    @body = body
  end

  # Get a query parameter (GET equivalent).
  def param(name, default: '')
    @query_params.fetch(name.to_s, default).to_s
  end

  # Get a POST parameter.
  def post(name, default: '')
    @post_data.fetch(name.to_s, default).to_s
  end

  # Get a cookie value.
  def cookie(name, default: '')
    @cookies.fetch(name.to_s, default).to_s
  end

  # Get a request header.
  def header(name, default: '')
    @headers.fetch(name.to_s, default).to_s
  end

  # Get raw request body.
  def body_str
    @body.to_s
  end

  # Get an uploaded file hash.
  def file(name)
    @files[name.to_s]
  end

  # Build from Rack env hash (for real execution).
  def self.from_rack_env(env)
    rack_req = defined?(Rack::Request) ? Rack::Request.new(env) : nil
    return new unless rack_req

    new(
      query_params: rack_req.GET,
      post_data: rack_req.POST,
      cookies: rack_req.cookies,
      headers: env.select { |k, _| k.start_with?('HTTP_') }
               .transform_keys { |k| k.sub('HTTP_', '').tr('_', '-') },
      files: {},
      body: rack_req.body&.read
    )
  end
end

class BenchmarkResponse
  attr_reader :status, :body, :headers

  def initialize(status, body, headers = {})
    @status = status
    @body = body
    @headers = headers
  end

  def self.ok(body)
    new(200, body)
  end

  def self.json(data)
    new(200, JSON.generate(data), { 'Content-Type' => 'application/json' })
  end

  def self.html(body)
    new(200, body, { 'Content-Type' => 'text/html; charset=UTF-8' })
  end

  def self.error(msg, status = 500)
    new(status, msg)
  end

  def self.bad_request(msg)
    new(400, msg)
  end

  def self.redirect(url)
    new(302, '', { 'Location' => url })
  end
end

# ============================================================================
# Database helpers
# ============================================================================

# Get a database connection (for test cases that need SQL).
def get_db_connection
  @_db_connection ||= begin
    dsn = ENV['BENCHMARK_DSN'] || 'sqlite::memory:'
    if dsn.start_with?('postgres')
      PG.connect(dsn)
    else
      SQLite3::Database.new(':memory:')
    end
  end
end

# ============================================================================
# ActiveRecord-style mocks (for rails_api app)
# ============================================================================

module FakeActiveRecord
  class Relation
    attr_reader :results

    def initialize(results = [])
      @results = results
    end

    def to_a
      @results
    end
  end

  class Base
    class << self
      def connection
        @connection ||= Connection.new
      end

      # Unsafe: string interpolation in where clause.
      def where(condition, *binds)
        if condition.is_a?(String) && binds.empty?
          connection.execute(condition)
        elsif condition.is_a?(String)
          connection.execute_params(condition, binds)
        elsif condition.is_a?(Hash)
          # Safe: hash conditions are always parameterized
          Relation.new([condition])
        end
      end

      # Unsafe: raw SQL execution.
      def find_by_sql(sql, binds = [])
        if binds.empty?
          connection.execute(sql)
        else
          connection.execute_params(sql, binds)
        end
      end

      # Safe: returns specific columns.
      def pluck(*columns)
        Relation.new([])
      end

      # Safe: parameterized find.
      def find(id)
        Relation.new([{ id: id }])
      end

      # Safe: sanitize SQL fragment.
      def sanitize_sql_array(ary)
        ary.first.gsub('?', '%s') % ary[1..].map { |v| connection.quote(v) }
      end

      def order(clause)
        Relation.new([])
      end
    end
  end

  class Connection
    def execute(sql)
      db = get_db_connection
      db.respond_to?(:exec) ? db.exec(sql) : db.execute(sql)
    end

    def execute_params(sql, binds)
      db = get_db_connection
      if db.respond_to?(:exec_params)
        db.exec_params(sql, binds)
      else
        stmt = db.prepare(sql)
        stmt.execute(*binds)
      end
    end

    def quote(value)
      "'#{value.to_s.gsub("'", "''")}'"
    end
  end
end

# ============================================================================
# Rails-style helpers (for rails_api app)
# ============================================================================

class FakeParams
  def initialize(data = {})
    @data = data
    @permitted = false
  end

  def [](key)
    @data[key.to_s] || @data[key.to_sym]
  end

  def require(key)
    val = self[key]
    raise ArgumentError, "param is missing: #{key}" unless val
    val.is_a?(Hash) ? FakeParams.new(val) : val
  end

  def permit(*keys)
    permitted = @data.select { |k, _| keys.include?(k.to_sym) || keys.include?(k.to_s) }
    result = FakeParams.new(permitted)
    result.instance_variable_set(:@permitted, true)
    result
  end

  def permit!
    @permitted = true
    self
  end

  def permitted?
    @permitted
  end

  def to_h
    @data
  end

  def to_unsafe_h
    @data
  end
end

# ============================================================================
# Sinatra-style helpers
# ============================================================================

module FakeSinatra
  def self.params
    {}
  end

  def self.halt(status, body = '')
    raise "Halted: #{status} #{body}"
  end
end

# ============================================================================
# Utility
# ============================================================================

# CGI.escapeHTML equivalent for test cases that don't require 'cgi'.
def escape_html(text)
  text.to_s
    .gsub('&', '&amp;')
    .gsub('<', '&lt;')
    .gsub('>', '&gt;')
    .gsub('"', '&quot;')
    .gsub("'", '&#39;')
end

# Rails-style h() helper alias.
alias h escape_html
