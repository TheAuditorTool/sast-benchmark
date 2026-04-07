require_relative 'shared'

FEED_ALLOWED_HOSTS = %w[feeds.example.com rss.partner.org].freeze

def fetch_feed(req)
  feed_url = req.param('feed_url')
  parsed = URI.parse(feed_url)
  return BenchmarkResponse.bad_request('host not allowed') unless FEED_ALLOWED_HOSTS.include?(parsed.host)
  content = Net::HTTP.get(parsed)
  BenchmarkResponse.ok(content)
end
