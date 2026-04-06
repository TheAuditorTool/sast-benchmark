require_relative 'shared'

# vuln-code-snippet start ruby_hardcoded_dotenv
def connect_database(req)
  db_url = ENV.fetch('DATABASE_URL') # vuln-code-snippet safe-line ruby_hardcoded_dotenv
  BenchmarkResponse.ok("connected to #{db_url.split('@').last}")
end
# vuln-code-snippet end ruby_hardcoded_dotenv
