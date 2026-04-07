require_relative 'shared'

VARY_OPTS = %w[Accept Accept-Encoding Accept-Language].freeze

def set_vary_allowlisted(req)
  h = req.param('vary')
  vary_value = VARY_OPTS.include?(h) ? h : 'Accept'
  response = BenchmarkResponse.ok('ok')
  response.headers['Vary'] = vary_value
  response
end
