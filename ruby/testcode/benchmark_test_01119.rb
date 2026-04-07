require_relative 'shared'

module Curb
  class Easy
    def self.perform(url); new(url); end
    def initialize(url); @url = url; end
    def body_str; @url; end
  end
end

def fetch_curb(req)
  url = req.param('url')
  response = Curb::Easy.perform(url)
  BenchmarkResponse.ok(response.body_str)
end
