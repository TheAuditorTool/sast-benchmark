require_relative 'shared'

def invalidate_cache_entry(req)
  key = req.param('key')
  File.delete("/cache/#{key}")
  BenchmarkResponse.ok("Cache entry removed")
end
