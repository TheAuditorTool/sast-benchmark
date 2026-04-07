require_relative 'shared'

def set_cache(req)
  cache_val = req.param('cache')
  BenchmarkResponse.new(200, 'ok', { 'Cache-Control' => cache_val })
end
