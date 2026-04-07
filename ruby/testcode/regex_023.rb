require_relative 'shared'

# vuln-code-snippet start ruby_regex_scan_tainted
def handle_scan_tainted(req)
  log_data = 'ERROR 500: internal server error at 2024-01-01T00:00:00Z WARN disk full INFO started'
  search = req.param('search')
  matches = log_data.scan(Regexp.new(search)) # vuln-code-snippet vuln-line ruby_regex_scan_tainted
  BenchmarkResponse.json({ matches: matches })
end
# vuln-code-snippet end ruby_regex_scan_tainted
