require_relative 'shared'

module SSRFFilterClient
  def self.get(url); "safe:#{url}"; end
end

def fetch_filtered(req)
  SSRFFilterClient.get(req.param('url'))
  BenchmarkResponse.json({ ok: true })
end
