require_relative 'shared'

begin; require 'async/http'; rescue LoadError; end

def fetch_async(req)
  url = req.param('url')
  Async::HTTP::Internet.new.get(url)
  BenchmarkResponse.json({ ok: true })
end
