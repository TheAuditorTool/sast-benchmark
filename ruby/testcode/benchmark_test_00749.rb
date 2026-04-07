require_relative 'shared'

def handle_scan_tainted(req)
  log_data = 'ERROR 500: internal server error at 2024-01-01T00:00:00Z WARN disk full INFO started'
  search = req.param('search')
  matches = log_data.scan(Regexp.new(search))
  BenchmarkResponse.json({ matches: matches })
end
