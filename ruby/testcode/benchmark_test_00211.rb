require_relative 'shared'

module NewRelicAgent
  def self.add_custom_attributes(attrs)
    $stdout.puts("newrelic_attrs: #{attrs.inspect}")
  end
end

def log_newrelic_custom(req)
  NewRelicAgent.add_custom_attributes({ event: req.param('event') })
  BenchmarkResponse.json({ ok: true })
end
