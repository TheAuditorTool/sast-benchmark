require_relative 'shared'

def show_search_results(req)
  query = h(req.param('q'))
  BenchmarkResponse.html("<html><body><p>Results for: #{query}</p></body></html>")
end
