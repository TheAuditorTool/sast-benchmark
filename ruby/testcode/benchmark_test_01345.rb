require_relative 'shared'

def fetch_feed(req)
  feed_url = req.param('feed_url')
  content = URI.open(feed_url).read
  BenchmarkResponse.ok(content)
end
