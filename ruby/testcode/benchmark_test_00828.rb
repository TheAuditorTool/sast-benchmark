require_relative 'shared'

def api_update(req)
  data = JSON.parse(req.body_str) rescue {}
  BenchmarkResponse.json({ updated: data })
end
